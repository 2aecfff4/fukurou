use qmetaobject::{QObject, QPointer};

///
pub fn queued_callback<T, Arg, F>(object: &T, mut callback: F) -> impl Fn(Arg) + Send + Sync + Clone
where
    T: QObject + 'static,
    Arg: Send,
    F: FnMut(&T, Arg) + 'static,
{
    let q_pointer = QPointer::from(object);
    qmetaobject::queued_callback(move |arg| {
        if let Some(this) = q_pointer.as_pinned() {
            let this = this.borrow();
            callback(this, arg);
        }
    })
}

///
pub fn queued_callback_mut<T, Arg, F>(
    object: &T,
    mut callback: F,
) -> impl Fn(Arg) + Send + Sync + Clone
where
    T: QObject + 'static,
    Arg: Send,
    F: FnMut(&mut T, Arg) + 'static,
{
    let q_pointer = QPointer::from(object);
    qmetaobject::queued_callback(move |arg| {
        if let Some(this) = q_pointer.as_pinned() {
            let mut this = this.borrow_mut();
            callback(&mut this, arg);
        }
    })
}
