use cedar_policy::*;
use std::collections::{HashMap, HashSet};
use std::usize;

const POLICY_SRC: &str = r#"
    permit(principal == User::"alice", action == Action::"view", resource == File::"93");
"#;

/// Calls [`stacker::remaining_stack`] then returns the result
///
/// This always returns [`None`] in the android bindings
pub fn get_remaining_stack() -> Option<usize> {
    stacker::remaining_stack()
}

pub fn authorize(principal: &str, action: &str, resource: &str) -> String {
    let policy: PolicySet = POLICY_SRC.parse().unwrap();

    let action = action.parse().expect("parse action uid");
    let principal = principal.parse().expect("parse principal uid");
    let file = resource.parse().expect("parse resource uid");
    let request =
        Request::new(principal, action, file, Context::empty(), None).expect("validate request");

    let test_entity = "Test::\"abc\"".parse().expect("parse resource uid");
    let test_entity_attrs = HashMap::from([
        (
            "item1".to_string(),
            RestrictedExpression::new_string("abc".to_string()),
        ),
        ("attr2".to_string(), RestrictedExpression::new_long(123)),
    ]);

    // This call to Entity::new is what fails in the android binding
    let test_entity =
        Entity::new(test_entity, test_entity_attrs, HashSet::new()).expect("build entity");

    let entities = Entities::from_entities([test_entity], None).expect("collect entities");
    let authorizer = Authorizer::new();
    let answer = authorizer.is_authorized(&request, &policy, &entities);

    match answer.decision() {
        Decision::Allow => "ALLOW",
        Decision::Deny => "DENY",
    }
    .to_string()
}

#[cfg(test)]
mod test {
    use crate::authorize;

    use super::get_remaining_stack;

    #[test]
    fn can_get_remaining_stack() {
        let remaining_stack = get_remaining_stack();
        assert!(
            matches!(remaining_stack, Some(stack_size) if stack_size > 0),
            "should get remaining stack size and it should be greater than 0: {:?}",
            remaining_stack,
        )
    }

    #[test]
    fn can_authorize_allow() {
        let result = authorize(r#"User::"alice""#, r#"Action::"view""#, r#"File::"93""#);
        assert_eq!(&result, "ALLOW");
    }

    #[test]
    fn can_authorize_deny() {
        let result = authorize(r#"User::"alice""#, r#"Action::"view""#, r#"File::"39""#);
        assert_eq!(&result, "DENY");
    }
}
