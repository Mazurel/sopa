use std::collections::HashMap;

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

#[function_component(Notification)]
fn notification(props: &NotificationProps) -> Html {
    let NotificationProps {
        notification_type,
        notification_content,
        request_close_cb,
    } = props;

    let onclick = {
        let request_close_cb = request_close_cb.clone();
        Callback::from(move |_| request_close_cb.emit(()))
    };

    match notification_type {
        NotificationType::Info => html! {
            <div class="notification is-info">
                <button class="delete" {onclick}></button>
                { notification_content }
            </div>
        },
        NotificationType::Error => html! {
            <div class="notification is-error">
                <button class="delete" {onclick}></button>
                { notification_content }
            </div>
        },
        NotificationType::Warning => html! {
            <div class="notification is-warning">
                <button class="delete" {onclick}></button>
                { notification_content }
            </div>
        },
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

    pub fn notify_warning<Msg: Into<String>>(&self, notification_content: Msg) {
        self.notify(notification_content, NotificationType::Warning);
    }
}
