use libmars::Client;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::*;
use crate::layouts::*;

pub struct Workspace<C: Client> {
    _num: u32,
    name: &'static str,
    clients: VecDeque<Rc<RefCell<C>>>,
    win_area: Dimensions,
    cur_layout: LayoutType,
    nmain: u32,
}

impl<C: Client> Workspace<C> {
    pub fn new(_num: u32, name: &'static str, win_area: Dimensions) -> Workspace<C> {
        return Workspace {
            _num, name,
            clients: VecDeque::new(),
            win_area,
            cur_layout: LayoutType::Floating,
            nmain: 1,
        };
    }

    pub fn apply_layout(&self) {
        Layout::get(self.cur_layout).apply_layout(self.win_area, &self.clients, self.nmain);
    }

    pub fn cycle_layout(&mut self) {
        let cur_idx = LAYOUT_TYPES.iter().position(|l| *l == self.cur_layout).unwrap();
        self.cur_layout = LAYOUT_TYPES[(cur_idx + 1) % LAYOUT_TYPES.len()];
        self.apply_layout();
    }

    pub fn dec_nmain(&mut self) {
        if self.nmain > 0 {
            self.nmain -= 1;
            self.apply_layout();
        }
    }

    pub fn inc_nmain(&mut self) {
        self.nmain += 1;
        self.apply_layout();
    }

    pub fn name(&self) -> &'static str {
        return self.name;
    }

    pub fn pull_front(&mut self, client_rc: Rc<RefCell<C>>) {
        let mut index_option = None;
        if let Some(index) = self.clients.iter().position(|c| c == &client_rc) {
            index_option = Some(index);
        }
        if let Some(index) = index_option {
            self.clients.remove(index);
            self.clients.push_front(client_rc);
            self.apply_layout();
        }
    }

    pub fn update_window_area(&mut self, win_area: Dimensions) {
        self.win_area = win_area;
        self.apply_layout();
    }
}

impl<C: Client> ClientList<C> for Workspace<C> {
    fn attach_client(&mut self, client_rc: Rc<RefCell<C>>) {
        self.clients.push_front(client_rc);
    }

    fn clients(&self) -> Box<dyn Iterator<Item = &Rc<RefCell<C>>> + '_> {
        return Box::new(self.clients.iter());
    }

    fn detach_client(&mut self, client_rc: &Rc<RefCell<C>>) {
        let mut index_option = None;
        if let Some(index) = self.clients().position(|c| c == client_rc) {
            index_option = Some(index);
        }
        if let Some(index) = index_option {
            self.clients.remove(index);
        }
    }
}
