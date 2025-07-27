use std::collections::HashMap;

use gloo::timers::callback::Interval;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NotificationType {
    Info,
    Error,
    Warning,
}

#[derive(Properties, PartialEq, Clone)]
pub struct NotificationProps {
    notification_type: NotificationType,
    notification_content: String,
    request_close_cb: Callback<()>,
}

const PROGRESS_MILISECONDS_TIMEOUT: u32 = 5000;
const PROGRESS_MILISECONDS_STEP: u32 = 5;

#[function_component(Notification)]
fn notification(props: &NotificationProps) -> Html {
    let NotificationProps {
        notification_type,
        notification_content,
        request_close_cb,
    } = props;

    let progress_changed = use_force_update();
    let progress_miliseconds_ref = use_mut_ref(|| 0);
    let progress_miliseconds = progress_miliseconds_ref.borrow().clone();

    if progress_miliseconds >= PROGRESS_MILISECONDS_TIMEOUT {
        request_close_cb.emit(());
    }

    // Reference to the Interval object to manage periodically called closure
    let interval_ref = {
        let progress_miliseconds_ref = progress_miliseconds_ref.clone();
        use_mut_ref(|| {
            Some(Interval::new(PROGRESS_MILISECONDS_STEP, move || {
                let current_miliseconds = *(progress_miliseconds_ref.borrow());
                let next_miliseconds_state = current_miliseconds + PROGRESS_MILISECONDS_STEP;
                *(progress_miliseconds_ref.borrow_mut()) = next_miliseconds_state;
                progress_changed.force_update();
            }))
        })
    };

    // On click handler to request closing Notification + clean Interval object
    let onclick = {
        let request_close_cb = request_close_cb.clone();
        let interval_ref = interval_ref.clone();
        Callback::from(move |_| {
            let mut maybe_interval = interval_ref.borrow_mut();
            if maybe_interval.is_some() {
                maybe_interval.take().unwrap().cancel();
            }
            request_close_cb.emit(());
        })
    };

    let value = progress_miliseconds.to_string();
    let max = PROGRESS_MILISECONDS_TIMEOUT.to_string();
    let class = match notification_type {
        NotificationType::Info => classes! {
            "notification", "is-info"
        },
        NotificationType::Error => classes! {
            "notification", "is-error"
        },
        NotificationType::Warning => classes! {
            "notification", "is-warning"
        },
    };

    html! {
        <div {class}>
            <button class="delete" {onclick}></button>
            <div class="content notification-content">
                { notification_content }
            </div>
            <progress class="progress" {value} {max}></progress>
        </div>
    }
}

// TODO: For partial EQ only notifications should be compared
#[derive(Clone, PartialEq)]
pub struct NotificationManager {
    pub notifications: UseStateHandle<HashMap<usize, Html>>,
    pub notifications_counter: UseStateHandle<usize>,
}

impl NotificationManager {
    fn publish_notification(
        &self,
        notification_content: String,
        notification_type: NotificationType,
    ) {
        let notification_id = *self.notifications_counter;
        let request_close_cb = {
            let notifications = self.notifications.clone();
            Callback::from(move |_| {
                let mut notifications_new = (*notifications).clone();
                notifications_new.remove(&notification_id);
                notifications.set(notifications_new);
            })
        };

        let notification = html!(
            <Notification
                {notification_type}
                {notification_content}
                {request_close_cb}/>
        );
        let mut new_notifications = (*self.notifications).clone();
        new_notifications.insert(notification_id, notification);
        self.notifications.set(new_notifications);
        self.notifications_counter.set(notification_id + 1);
    }

    pub fn notify<Msg: Into<String>>(
        &self,
        notification_content: Msg,
        notification_type: NotificationType,
    ) {
        self.publish_notification(notification_content.into(), notification_type);
    }

    pub fn notify_info<Msg: Into<String>>(&self, notification_content: Msg) {
        self.notify(notification_content, NotificationType::Info);
    }

    pub fn notify_error<Msg: Into<String>>(&self, notification_content: Msg) {
        self.notify(notification_content, NotificationType::Error);
    }

    #[allow(unused)]
    pub fn notify_warning<Msg: Into<String>>(&self, notification_content: Msg) {
        self.notify(notification_content, NotificationType::Warning);
    }
}
