use priority_queue::PriorityQueue;
use syncrim::common::{Component,Input, Condition, Id, OutputType, Ports, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
use std::{convert::Into, cell::RefCell, cmp::Reverse};
#[derive(Serialize, Deserialize)]
pub struct Antiq {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) pop_i: Input,
    pub(crate) drop_i: Input,
    pub(crate) push_i: Input,
    pub(crate) drop_id_i: Input,
    pub(crate) push_id_i: Input,
    pub(crate) data_i: Input,
    pub(crate) sysclk: Input,

    //CONSTANTS
    pub(crate) DEPTH:u8,
    pub(crate) DATA_WIDTH:u8,

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
                vec![&self.pop_i, &self.drop_i, &self.push_i, &self.drop_id_i,&self.push_id_i, &self.data_i],
                OutputType::Combinatorial,
                vec!["push_rdy_o", "pop_rdy_o", "drop_rdy_o", "full_o", "empty_o", "cnt_o", "data_o", "peek_vld_o", "peek_data_o", "overflow_o", "data_overflow_o"],
            ),
        )
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        //simulator.set_out_value(&self.id, "out", self.value.get_value());
        let pop_i:u32 = simulator.get_input_value(&self.pop_i).try_into().unwrap();
        let drop_i:u32 = simulator.get_input_value(&self.drop_i).try_into().unwrap();
        let push_i:u32 = simulator.get_input_value(&self.push_i).try_into().unwrap();
        let drop_id_i:u32 = simulator.get_input_value(&self.drop_id_i).try_into().unwrap();
        let data_i:u32 = simulator.get_input_value(&self.data_i).try_into().unwrap();
        let push_id_i:u32 = simulator.get_input_value(&self.push_id_i).try_into().unwrap();
        let mut queue = self.queue.borrow_mut();
        if push_i == 1 {
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
            trace!("pushing deadline:{} id:{}", data_i, push_id_i);
            queue.push(push_id_i, Reverse(data_i));
            simulator.set_out_value(&self.id, "push_rdy_o", 1);
        }
        else{
            simulator.set_out_value(&self.id, "push_rdy_o", 0);
        }
        if pop_i == 1 {
            if !queue.is_empty(){
                let item = queue.pop().clone().unwrap();
                simulator.set_out_value(&self.id, "data_o", item.0);
                simulator.set_out_value(&self.id, "pop_rdy_o", 1);
            }

        }
        if drop_i == 1 {
            queue.remove(&drop_id_i);
            simulator.set_out_value(&self.id, "drop_rdy_o", 1);
        }
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
            trace!("DEADLINE:{}, ID:{}", item.1.0, item.0);
        }

        Ok(())
    }
}

impl Antiq {
    pub fn new(id: &str, pos: (f32, f32), pop_i: Input, drop_i: Input,
    push_i: Input, drop_id_i: Input, push_id_i:Input, data_i: Input,sysclk:Input, depth:u8, data_width:u8) -> Self {
        Antiq {
            id: id.to_string(),
            pos,
            pop_i: pop_i.into(),
            drop_i: drop_i.into(),
            push_i: push_i.into(),
            drop_id_i: drop_id_i.into(),
            data_i: data_i.into(),
            push_id_i: push_id_i.into(),
            sysclk: sysclk.into(),
            DEPTH: depth,
            DATA_WIDTH: data_width,
            queue: RefCell::new(PriorityQueue::new()),
        }
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

}
