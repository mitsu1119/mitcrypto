#[allow(unused_macros)]
#[macro_export]
macro_rules! prog_log {
    ($name:ident, $content_type: ty) => {
        mod $name {
            use std::cell::RefCell;
            use std::rc::Rc;
            type ContentType = $content_type;
            thread_local!(static LOG: Rc<RefCell<Vec<ContentType>>> = Rc::new(RefCell::new(vec![])));
            // ログを丸々取得
            pub fn get() -> Rc<RefCell<Vec<ContentType>>> { LOG.with(|log| log.clone())}
            // ログの追加
            #[inline]
            #[allow(dead_code)]
            pub fn push(block: ContentType) { get().borrow_mut().push(block);}
            // ログのポップ
            #[inline]
            #[allow(dead_code)]
            pub fn pop() -> Option<ContentType> { get().borrow_mut().pop()}
            // クリア
            #[inline]
            #[allow(dead_code)]
            pub fn clear() { get().borrow_mut().clear(); }
        }
    };
}
