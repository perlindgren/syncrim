use priority_queue::PriorityQueue;
use syncrim::common::{Component,Input, Condition, Id, OutputType, Ports, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
use std::{convert::Into, cell::RefCell, cmp::Reverse};
#[derive(Serialize, Deserialize)]
pub struct Antiq {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) sysclk: Input,
    pub(crate) we: Input,
    pub(crate) d_in: Input,
    pub(crate) addr: Input,

    //CONSTANTS
    pub(crate) DEPTH:u8,
    pub(crate) DATA_WIDTH:u8,

    //mmio constants
    pub(crate) RANGE_LOW:u32,
    pub(crate) RANGE_HIGH:u32,

    //Internal state
    pub queue: RefCell<PriorityQueue<u32, Reverse<u32>>> 
}

#[typetag::serde]
impl Component for Antiq {
    fn to_(&self) {
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // Constants do not take any inputs
                vec![&self.sysclk, &self.we, &self.d_in, &self.addr],
                OutputType::Combinatorial,
                vec!["push_rdy_o", "pop_rdy_o", "drop_rdy_o", "full_o", "empty_o", "cnt_o", "data_o", "peek_vld_o", "peek_data_o", "overflow_o", "data_overflow_o", "int_id"],
            ),
        )
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        //AntiqCtl part
        let sysclk:u32 = simulator.get_input_value(&self.sysclk).try_into().unwrap();
        let we:u32 = simulator.get_input_value(&self.we).try_into().unwrap();
        let d_in:u32 = simulator.get_input_value(&self.d_in).try_into().unwrap();
        let addr:u32 = simulator.get_input_value(&self.addr).try_into().unwrap();
        let interrupt_id = d_in & 0xFF;   // shift out time data
        let time_in = d_in >> 8; // mask out interrupt id
        let mut data_out = 0;
        let mut push = false;
        let mut drop = false;
        let mut pop = false;
        let mut data_to_push = 0;
        if we == 1 && self.within_range(addr) {
            match addr - self.RANGE_LOW{
                0 =>{//relative time maybe ignore this for now, needs signal to antiq to trigger
                  //adding sysclk
                       // data_out = (sysclk + time_in)<<8 | interrupt_id;
                        trace!("relative push");
                        push = true;
                        data_to_push = ((sysclk+time_in)<<8) | interrupt_id;
                    }
                4 =>{//absolute time
                        trace!("absolute push");
                        push = true;
                        data_to_push = (time_in<<8) | interrupt_id;
                        trace!("time_in:{}", time_in);
                        trace!("interrupt_id:{}", interrupt_id);
                    }
                8 =>{//drop input/push output
                        drop = true;
                    } 
                int=>{//do nothing, we've checked that addr is within range
                    trace!("addr diff{}", int)
                    }
            }
        }
        let mut queue = self.queue.borrow_mut();
        if !queue.is_empty(){
            if queue.peek().unwrap().1.0 >> 8 <= sysclk {
                pop = true;
            }
        }


        //Actual AntiQ
        if push{
            // if the queue size is exceeded, signal overflow, and pop from the queue before
            // pushing
            if queue.len() == self.DEPTH as usize{
                let cloned = queue.clone();
                let last = cloned.into_sorted_iter().last().clone().unwrap().0;
                let item = queue.remove(&last).unwrap();
                simulator.set_out_value(&self.id, "overflow_o", 1);
                simulator.set_out_value(&self.id, "data_overflow_o", item.0);
            }
            // push the item to the queue
            trace!("pushing release:{} id:{}", data_to_push >> 8, sysclk);
            queue.push(sysclk, Reverse(data_to_push));
            simulator.set_out_value(&self.id, "push_rdy_o", 1);
        }
        else{
            simulator.set_out_value(&self.id, "push_rdy_o", 0);
        }
        if pop {
            if !queue.is_empty(){
                let item = queue.pop().clone().unwrap();
                trace!("Popping: int id:{}", (item.1.0 & 0xFF));
                simulator.set_out_value(&self.id, "int_id", item.1.0 & 0xFF);
                simulator.set_out_value(&self.id, "pop_rdy_o", 1);
            }

        }
        else{
            simulator.set_out_value(&self.id, "pop_rdy_o", 0);
        }
        /*
        if drop_i == 1 {
            queue.remove(&drop_id_i);
            simulator.set_out_value(&self.id, "drop_rdy_o", 1);
        }*/
        if queue.len() == self.DEPTH as usize{
            simulator.set_out_value(&self.id, "full_o", 1);
        }
        else{
            simulator.set_out_value(&self.id, "full_o", 0);
        }
        if queue.is_empty(){
            simulator.set_out_value(&self.id, "empty_o", 1);
            simulator.set_out_value(&self.id, "peek_vld_o", 0)
        }
        else{
            simulator.set_out_value(&self.id, "empty_o", 0);
            simulator.set_out_value(&self.id, "peek_vld_o", 1);
            simulator.set_out_value(&self.id, "peek_data_o", *queue.peek().unwrap().0); // infallible since queue
            // is not empty
        }


        simulator.set_out_value(&self.id, "cnt_o", queue.len() as u32);
        trace!("QUEUE:");
        for item in queue.iter(){
            trace!("RELEASE:{}, INTERRUPT ID:{}, ID:{}", item.1.0 >> 8,item.1.0&0xFF, item.0);
        }

        Ok(())
    }
}

impl Antiq {
    pub fn new(id: &str, pos: (f32, f32), sysclk:Input, depth:u8, data_width:u8,
    we:Input, d_in: Input, addr: Input, range_low:u32,) -> Self {
        Antiq {
            id: id.to_string(),
            pos,
            sysclk: sysclk.into(),
            DEPTH: depth,
            DATA_WIDTH: data_width,
            queue: RefCell::new(PriorityQueue::new()),
            we: we,
            d_in: d_in,
            addr: addr,
            RANGE_LOW: range_low,
            RANGE_HIGH: range_low + 3*4,
        }
    }
    fn within_range(&self, addr:u32)->bool{
        self.RANGE_LOW <= addr && addr <= self.RANGE_HIGH
    }
}
mod test{
#![allow(unused_imports)]
use super::*;
use std::rc::Rc;
use syncrim::{
    common::{ComponentStore, Input, Simulator},
    components::ProbeOut,
};
    #[test]
    fn test_push() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new(
                "pop_i",
                )),
                Rc::new(ProbeOut::new(
                "drop_i",
                )),
                Rc::new(ProbeOut::new(
                "push_i",
                )),
                Rc::new(ProbeOut::new(
                "drop_id_i",
                )),
                Rc::new(ProbeOut::new(
                "push_id_i"
                )),
                Rc::new(ProbeOut::new("data_i")),
//(id: &str, pos: (f32, f32), sysclk:Input, depth:u8, data_width:u8,
//    we:Input, d_in: Input, addr: Input, range_low:u32,)
                Rc::new(Antiq::new(
                    "antiq",
                    (400.0, 400.0),
                    Input::new("sysclk", "out"),
                    3,
                    32,
                    Input::new("we", "out"),
                    Input::new("d_in", "out"),
                    Input::new("addr", "out"), 
                    0, //mmio starts at 0x0
                )),

            ],
        };
        let mut simulator = Simulator::new(&cs);
        let push_rdy_o = &Input::new("antiq", "push_rdy_o");
        let pop_rdy_o = &Input::new("antiq", "pop_rdy_o");
        let drop_rdy_o = &Input::new("antiq", "drop_rdy_o");
        let full_o = &Input::new("antiq", "full_o");
        let empty_o = &Input::new("antiq", "empty_o");
        let cnt_o = &Input::new("antiq", "cnt_o");
        let data_o = &Input::new("antiq", "data_o");
        let peek_vld_o = &Input::new("antiq", "peek_vld_o");
        let peek_data_o = &Input::new("antiq", "peek_data_o");
        let overflow_o = &Input::new("antiq", "overflow_o");
        let data_overflow_o = &Input::new("antiq", "data_overflow_o");

        simulator.set_out_value("we", "out", 0);
        simulator.set_out_value("d_in", "out", 0);
        simulator.set_out_value("addr", "out", 1);
        simulator.clock();
        assert_eq!(simulator.get_input_value(push_rdy_o), 1.try_into().unwrap());
        assert_eq!(simulator.get_input_value(pop_rdy_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(drop_rdy_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(full_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(empty_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(cnt_o), 1.try_into().unwrap());
        assert_eq!(simulator.get_input_value(data_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(peek_vld_o), 1.try_into().unwrap());
        assert_eq!(simulator.get_input_value(peek_data_o), 98.try_into().unwrap());
        assert_eq!(simulator.get_input_value(overflow_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(data_overflow_o), 0.try_into().unwrap());

        simulator.set_out_value("pop_i", "out", 0);
        simulator.set_out_value("drop_i", "out", 0);
        simulator.set_out_value("push_i", "out", 1);
        simulator.set_out_value("drop_id_i", "out", 0);
        simulator.set_out_value("push_id_i", "out", 100);
        simulator.set_out_value("data_i", "out", 1);
        simulator.clock();
        assert_eq!(simulator.get_input_value(push_rdy_o), 1.try_into().unwrap());
        assert_eq!(simulator.get_input_value(pop_rdy_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(drop_rdy_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(full_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(empty_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(cnt_o), 2.try_into().unwrap());
        assert_eq!(simulator.get_input_value(data_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(peek_vld_o), 1.try_into().unwrap());
        assert_eq!(simulator.get_input_value(peek_data_o), 100.try_into().unwrap());
        assert_eq!(simulator.get_input_value(overflow_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(data_overflow_o), 0.try_into().unwrap());

        simulator.set_out_value("pop_i", "out", 0);
        simulator.set_out_value("drop_i", "out", 0);
        simulator.set_out_value("push_i", "out", 1);
        simulator.set_out_value("drop_id_i", "out", 0);
        simulator.set_out_value("push_id_i", "out", 102);
        simulator.set_out_value("data_i", "out", 4);
        simulator.clock();
        assert_eq!(simulator.get_input_value(push_rdy_o), 1.try_into().unwrap());
        assert_eq!(simulator.get_input_value(pop_rdy_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(drop_rdy_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(full_o), 1.try_into().unwrap());
        assert_eq!(simulator.get_input_value(empty_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(cnt_o), 3.try_into().unwrap());
        assert_eq!(simulator.get_input_value(data_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(peek_vld_o), 1.try_into().unwrap());
        assert_eq!(simulator.get_input_value(peek_data_o), 100.try_into().unwrap());
        assert_eq!(simulator.get_input_value(overflow_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(data_overflow_o), 0.try_into().unwrap());

        simulator.set_out_value("pop_i", "out", 0);
        simulator.set_out_value("drop_i", "out", 0);
        simulator.set_out_value("push_i", "out", 1);
        simulator.set_out_value("drop_id_i", "out", 0);
        simulator.set_out_value("push_id_i", "out", 104);
        simulator.set_out_value("data_i", "out", 3);
        simulator.clock();
        assert_eq!(simulator.get_input_value(push_rdy_o), 1.try_into().unwrap());
        assert_eq!(simulator.get_input_value(pop_rdy_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(drop_rdy_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(full_o), 1.try_into().unwrap());
        assert_eq!(simulator.get_input_value(empty_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(cnt_o), 3.try_into().unwrap());
        assert_eq!(simulator.get_input_value(data_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(peek_vld_o), 1.try_into().unwrap());
        assert_eq!(simulator.get_input_value(peek_data_o), 100.try_into().unwrap());
        assert_eq!(simulator.get_input_value(overflow_o), 1.try_into().unwrap());
        assert_eq!(simulator.get_input_value(data_overflow_o), 102.try_into().unwrap());

    }
    /*
    #[test]
    fn test_pop() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new(
                "pop_i",
                )),
                Rc::new(ProbeOut::new(
                "drop_i",
                )),
                Rc::new(ProbeOut::new(
                "push_i",
                )),
                Rc::new(ProbeOut::new(
                "drop_id_i",
                )),
                Rc::new(ProbeOut::new(
                "push_id_i"
                )),
                Rc::new(ProbeOut::new("data_i")),
                Rc::new(Antiq::new(
                    "antiq",
                    (400.0, 400.0),
                    Input::new("pop_i", "out"),
                    Input::new("drop_i", "out"),
                    Input::new("push_i", "out"),
                    Input::new("drop_id_i", "out"),
                    Input::new("push_id_i", "out"),
                    Input::new("data_i", "out"),
                    Input::new("data_i", "out"),
    
                    3, //depth 3
                    32, //32 bit wide data
                )),

            ],
        };
        let mut simulator = Simulator::new(&cs);
        let push_rdy_o = &Input::new("antiq", "push_rdy_o");
        let pop_rdy_o = &Input::new("antiq", "pop_rdy_o");
        let drop_rdy_o = &Input::new("antiq", "drop_rdy_o");
        let full_o = &Input::new("antiq", "full_o");
        let empty_o = &Input::new("antiq", "empty_o");
        let cnt_o = &Input::new("antiq", "cnt_o");
        let data_o = &Input::new("antiq", "data_o");
        let peek_vld_o = &Input::new("antiq", "peek_vld_o");
        let peek_data_o = &Input::new("antiq", "peek_data_o");
        let overflow_o = &Input::new("antiq", "overflow_o");
        let data_overflow_o = &Input::new("antiq", "data_overflow_o");

        simulator.set_out_value("pop_i", "out", 0);
        simulator.set_out_value("drop_i", "out", 0);
        simulator.set_out_value("push_i", "out", 1);
        simulator.set_out_value("drop_id_i", "out", 0);
        simulator.set_out_value("push_id_i", "out", 98);
        simulator.set_out_value("data_i", "out", 2);
        simulator.clock();
        simulator.set_out_value("pop_i", "out", 0);
        simulator.set_out_value("drop_i", "out", 0);
        simulator.set_out_value("push_i", "out", 1);
        simulator.set_out_value("drop_id_i", "out", 0);
        simulator.set_out_value("push_id_i", "out", 100);
        simulator.set_out_value("data_i", "out", 1);
        simulator.clock();
        simulator.set_out_value("pop_i", "out", 0);
        simulator.set_out_value("drop_i", "out", 0);
        simulator.set_out_value("push_i", "out", 1);
        simulator.set_out_value("drop_id_i", "out", 0);
        simulator.set_out_value("push_id_i", "out", 102);
        simulator.set_out_value("data_i", "out", 3);
        simulator.clock(); //fill the queue with values
        simulator.set_out_value("pop_i", "out", 1);
        simulator.set_out_value("drop_i", "out", 0);
        simulator.set_out_value("push_i", "out", 0);
        simulator.set_out_value("drop_id_i", "out", 0);
        simulator.set_out_value("push_id_i", "out", 0);
        simulator.set_out_value("data_i", "out", 0);
        simulator.clock();
        assert_eq!(simulator.get_input_value(push_rdy_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(pop_rdy_o), 1.try_into().unwrap());
        assert_eq!(simulator.get_input_value(drop_rdy_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(full_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(empty_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(cnt_o), 2.try_into().unwrap());
        assert_eq!(simulator.get_input_value(data_o), 100.try_into().unwrap());
        assert_eq!(simulator.get_input_value(peek_vld_o), 1.try_into().unwrap());
        assert_eq!(simulator.get_input_value(peek_data_o), 98.try_into().unwrap());
        assert_eq!(simulator.get_input_value(overflow_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(data_overflow_o), 0.try_into().unwrap());
    }
    #[test]
    fn test_drop(){
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new(
                "pop_i",
                )),
                Rc::new(ProbeOut::new(
                "drop_i",
                )),
                Rc::new(ProbeOut::new(
                "push_i",
                )),
                Rc::new(ProbeOut::new(
                "drop_id_i",
                )),
                Rc::new(ProbeOut::new(
                "push_id_i"
                )),
                Rc::new(ProbeOut::new("data_i")),
                Rc::new(Antiq::new(
                    "antiq",
                    (400.0, 400.0),
                    Input::new("pop_i", "out"),
                    Input::new("drop_i", "out"),
                    Input::new("push_i", "out"),
                    Input::new("drop_id_i", "out"),
                    Input::new("push_id_i", "out"),
                    Input::new("data_i", "out"),
                    Input::new("data_i", "out"),
    
                    3, //depth 3
                    32, //32 bit wide data
                )),

            ],
        };
        let mut simulator = Simulator::new(&cs);
        let push_rdy_o = &Input::new("antiq", "push_rdy_o");
        let pop_rdy_o = &Input::new("antiq", "pop_rdy_o");
        let drop_rdy_o = &Input::new("antiq", "drop_rdy_o");
        let full_o = &Input::new("antiq", "full_o");
        let empty_o = &Input::new("antiq", "empty_o");
        let cnt_o = &Input::new("antiq", "cnt_o");
        let data_o = &Input::new("antiq", "data_o");
        let peek_vld_o = &Input::new("antiq", "peek_vld_o");
        let peek_data_o = &Input::new("antiq", "peek_data_o");
        let overflow_o = &Input::new("antiq", "overflow_o");
        let data_overflow_o = &Input::new("antiq", "data_overflow_o");

        simulator.set_out_value("pop_i", "out", 0);
        simulator.set_out_value("drop_i", "out", 0);
        simulator.set_out_value("push_i", "out", 1);
        simulator.set_out_value("drop_id_i", "out", 0);
        simulator.set_out_value("push_id_i", "out", 98);
        simulator.set_out_value("data_i", "out", 2);
        simulator.clock();
        simulator.set_out_value("pop_i", "out", 0);
        simulator.set_out_value("drop_i", "out", 0);
        simulator.set_out_value("push_i", "out", 1);
        simulator.set_out_value("drop_id_i", "out", 0);
        simulator.set_out_value("push_id_i", "out", 100);
        simulator.set_out_value("data_i", "out", 1);
        simulator.clock();
        simulator.set_out_value("pop_i", "out", 0);
        simulator.set_out_value("drop_i", "out", 0);
        simulator.set_out_value("push_i", "out", 1);
        simulator.set_out_value("drop_id_i", "out", 0);
        simulator.set_out_value("push_id_i", "out", 102);
        simulator.set_out_value("data_i", "out", 3);
        simulator.clock(); //fill the queue with values
        simulator.set_out_value("pop_i", "out", 0);
        simulator.set_out_value("drop_i", "out", 1);
        simulator.set_out_value("push_i", "out", 0);
        simulator.set_out_value("drop_id_i", "out", 98);
        simulator.set_out_value("push_id_i", "out", 0);
        simulator.set_out_value("data_i", "out", 0);
        simulator.clock();

        assert_eq!(simulator.get_input_value(push_rdy_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(pop_rdy_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(drop_rdy_o), 1.try_into().unwrap());
        assert_eq!(simulator.get_input_value(full_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(empty_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(cnt_o), 2.try_into().unwrap());
        assert_eq!(simulator.get_input_value(data_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(peek_vld_o), 1.try_into().unwrap());
        assert_eq!(simulator.get_input_value(peek_data_o), 100.try_into().unwrap());
        assert_eq!(simulator.get_input_value(overflow_o), 0.try_into().unwrap());
        assert_eq!(simulator.get_input_value(data_overflow_o), 0.try_into().unwrap());

    } 
*/
}
