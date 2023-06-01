use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Arc;
use std::thread;

use bincode;

use uuid::Uuid;

#[derive(Clone)]
pub struct Listener {
    callback: Arc<dyn Fn(Vec<u8>) + Sync + Send + 'static>,
    id: String,
}

#[derive(Default, Clone)]
pub struct EventEmitter<Event>
where
    Event: PartialEq + Eq + Default + Clone + Hash + 'static,
{
    pub listeners: HashMap<Event, Vec<Listener>>,
    events: PhantomData<Event>,
}

impl<Event> EventEmitter<Event>
where
    Event: PartialEq + Eq + Default + Clone + Hash + 'static,
{
    pub fn new() -> Self {
        Self { ..Self::default() }
    }

    pub fn emit<T>(&mut self, event: Event, value: T) -> Vec<thread::JoinHandle<()>>
    where
        T: Serialize,
    {
        let mut callback_handlers: Vec<thread::JoinHandle<()>> = Vec::new();

        if let Some(listeners) = self.listeners.get_mut(&event) {
            let bytes: Vec<u8> = bincode::serialize(&value).unwrap();

            let listeners_to_remove: Vec<usize> = Vec::new();
            for listener in listeners.iter_mut() {
                let cloned_bytes = bytes.clone();
                let callback = Arc::clone(&listener.callback);

                callback_handlers.push(thread::spawn(move || {
                    callback(cloned_bytes);
                }));
            }

            // Reverse here so we don't mess up the ordering of the vector
            for index in listeners_to_remove.into_iter().rev() {
                listeners.remove(index);
            }
        }

        return callback_handlers;
    }

    pub fn remove_listener<S>(&mut self, id_to_delete: S) -> Option<String>
    where
        S: Into<String>,
    {
        let id = id_to_delete.into();
        for (_, event_listeners) in self.listeners.iter_mut() {
            if let Some(index) = event_listeners
                .iter()
                .position(|listener| listener.id == id)
            {
                event_listeners.remove(index);
                return Some(id);
            }
        }

        return None;
    }

    pub fn on<F, T>(&mut self, event: Event, callback: F) -> String
    where
        for<'de> T: Deserialize<'de>,
        F: Fn(T) + 'static + Sync + Send,
    {
        let id = Uuid::new_v4().to_string();
        let parsed_callback = move |bytes: Vec<u8>| {
            let value: T = bincode::deserialize(&bytes).unwrap();
            callback(value);
        };

        let listener = Listener {
            id: id.clone(),
            callback: Arc::new(parsed_callback),
        };

        match self.listeners.get_mut(&event) {
            Some(callbacks) => {
                callbacks.push(listener);
            }
            None => {
                self.listeners.insert(event, vec![listener]);
            }
        }

        return id;
    }
}
