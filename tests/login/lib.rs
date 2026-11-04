#![allow(dead_code)]
extern crate alloc;
#[cfg(test)]
#[path = "../../server/src/auth.rs"]
mod auth;
#[cfg(test)]
#[path = "../../server/src/startup/context.rs"]
mod context;
#[cfg(test)]
mod frame { pub use account_api::Request; }
#[cfg(test)]
mod tests {
    use crate::frame::Request;
    use env::PieToken;
    use env::wire::Span as _;
    #[test]
    fn account_password_requires_both_matching() {
        assert!(super::auth::verify(b"anran", b"sqware"));
        assert!(!super::auth::verify(b"anran", b"wrong"));
        assert!(!super::auth::verify(b"unknown", b"sqware"));
        assert!(!super::auth::verify(b"anran", b""));
    }
    #[test]
    fn requests_reject_invalid_shapes_and_trailing_bytes() {
        let cases = [
            ("anran", true),
            ("", false),
            ("../anran", false),
            ("other", true),
        ];
        for (account, valid) in cases {
            let request = Request {
                op: account_api::CREATE,
                id: system_api::control::construction::CreateId { value: 1 },
                account: account.into(),
                back: PieToken::NONE,
            };
            let mut bytes = [0; Request::LEN + 1];
            let n = request.store_at(&mut bytes, 0).unwrap();
            assert_eq!(Request::take(&bytes[..n]).is_some(), valid);
            assert!(Request::take(&bytes[..n + 1]).is_none());
            for end in 0..n {
                assert!(Request::take(&bytes[..end]).is_none());
            }
        }
    }
    #[test]
    fn startup_context_frame_matches_shell_wire_contract() {
        use env::TaskId;
        let system = TaskId::new(0x1122_3344_5566_7788);
        let mut frame = [0; crate::context::Context::LEN];
        let n = crate::context::Context { system }
            .store_at(&mut frame, 0)
            .expect("context frame fits");
        // The untouched Shell receiver in the Source tree
        // (programs/src/user/shell/startup.rs) pulls this frame whole: exactly
        // 8 bytes, one little-endian TaskId at offset 0, end == n, nonzero
        // system task. Pin those bytes here; this is a frame contract check,
        // not a native runtime claim.
        assert_eq!(crate::context::Context::LEN, 8);
        assert_eq!(n, 8);
        assert_eq!(frame, 0x1122_3344_5566_7788u64.to_le_bytes());
        assert_ne!(system.get(), 0);
        let (parsed, end) = crate::context::Context::fetch_at(&frame, 0)
            .expect("context frame parses");
        assert_eq!(end, 8);
        assert_eq!(parsed.system.get(), system.get());

        // Shell rejects the frame when `end != n`; a ninth byte is not consumed.
        let mut longer = [0; crate::context::Context::LEN + 1];
        let n = crate::context::Context { system }
            .store_at(&mut longer, 0)
            .expect("context frame fits");
        let (_, end) = crate::context::Context::fetch_at(&longer, 0)
            .expect("context frame parses");
        assert_eq!(n, 8);
        assert_ne!(end, longer.len());
    }
}
