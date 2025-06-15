use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NotificationType {
    Info,
}

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct NotificationProps {
    notification_type: NotificationType,
    content: String,
}
