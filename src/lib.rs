use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Debug)]
pub struct AMErrorEvent {
    workflow: String,
    exc_id: String,
    categories: Vec<String>,
    message: String,
    continue_url: Option<String>,
    abort_url: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub req_id: String,
    pub message: String,
}

pub fn format_recepients(recepients: &[String], prefix: &str) -> String {
    recepients
        .iter()
        .map(|category| format!("{}{}", prefix, category))
        .collect::<Vec<String>>()
        .join(", ")
}

#[cfg(test)]
mod tests_format_recepients {
    use super::*;

    #[test]
    fn test_format_recepients_empty() {
        let input = vec![];
        let expected_output = "".to_string();
        let prefix = "@";
        assert_eq!(format_recepients(&input, prefix), expected_output);
    }

    #[test]
    fn test_format_recepients_one_element() {
        let input = vec!["admin".to_string()];
        let expected_output = "&admin".to_string();
        let prefix = "&";
        assert_eq!(format_recepients(&input, prefix), expected_output);
    }

    #[test]
    fn test_format_recepients_multiple_elements() {
        let input = vec![
            "admin".to_string(),
            "developer".to_string(),
            "QA".to_string(),
        ];
        let expected_output = "$admin, $developer, $QA".to_string();
        let prefix = "$";
        assert_eq!(format_recepients(&input, prefix), expected_output);
    }
}

pub fn create_header_message(event: &AMErrorEvent) -> String {
    let categories_string = format_recepients(&event.categories, "@");
    format!(
        "Hi {}, The workflow: <b><font color='black'>{}</font></b>; \
    failed for the production ID: <b><font color='black'>{}</font></b>. \
    This is the error message: <font color='#FF0000'>{}</font>",
        categories_string, event.workflow, event.exc_id, event.message
    )
}

#[cfg(test)]
mod tests_create_header_message {
    use super::*;

    #[test]
    fn test_create_header_message_one_categorie() {
        let event = AMErrorEvent {
            workflow: "workflow1".to_string(),
            exc_id: "exc_id1".to_string(),
            categories: vec!["admin".to_string()],
            message: "Error message".to_string(),
            continue_url: None,
            abort_url: None,
        };

        let expected_output = format!(
            "Hi {}, The workflow: <b><font color='black'>{}</font></b>; \
    failed for the production ID: <b><font color='black'>{}</font></b>. \
    This is the error message: <font color='#FF0000'>{}</font>",
            "@admin", "workflow1", "exc_id1", "Error message"
        );

        let output = create_header_message(&event);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_create_header_message_multiple_categories() {
        let event = AMErrorEvent {
            workflow: "workflow2".to_string(),
            exc_id: "exc_id2".to_string(),
            categories: vec!["admin".to_string(), "developer".to_string()],
            message: "Another error message".to_string(),
            continue_url: None,
            abort_url: None,
        };

        let expected_output = format!(
            "Hi {}, The workflow: <b><font color='black'>{}</font></b>; \
    failed for the production ID: <b><font color='black'>{}</font></b>. \
    This is the error message: <font color='#FF0000'>{}</font>",
            "@admin, @developer", "workflow2", "exc_id2", "Another error message"
        );

        let output = create_header_message(&event);
        assert_eq!(output, expected_output);
    }
}

pub fn create_card_buttons(event: &AMErrorEvent) -> Vec<Value> {
    let mut buttons = vec![];

    if let Some(continue_url) = &event.continue_url {
        buttons.push(json!({
            "textButton": {
                "text": "Continue",
                "onClick": {
                    "openLink": {
                        "url": continue_url
                    }
                }
            }
        }));
    }
    if let Some(abort_url) = &event.abort_url {
        buttons.push(json!({
            "textButton": {
                "text": "Abort",
                "onClick": {
                    "openLink": {
                        "url": abort_url
                    }
                }
            }
        }));
    }
    buttons
}

#[cfg(test)]
mod tests_create_card_buttons {
    use super::*;

    #[test]
    fn test_create_card_buttons_continue_only() {
        let event = AMErrorEvent {
            workflow: "".to_string(),
            exc_id: "".to_string(),
            categories: vec![],
            message: "".to_string(),
            continue_url: Some("https://continue.com".to_string()),
            abort_url: None,
        };

        let expected_output = vec![
            json!({"textButton": {"text": "Continue","onClick": {"openLink": {"url": "https://continue.com"}}}}),
        ];

        let output = create_card_buttons(&event);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_create_card_buttons_abort_only() {
        let event = AMErrorEvent {
            workflow: "".to_string(),
            exc_id: "".to_string(),
            categories: vec![],
            message: "".to_string(),
            continue_url: None,
            abort_url: Some("https://abort.com".to_string()),
        };

        let expected_output = vec![
            json!({"textButton": {"text": "Abort","onClick": {"openLink": {"url": "https://abort.com"}}}}),
        ];

        let output = create_card_buttons(&event);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_create_card_buttons_both() {
        let event = AMErrorEvent {
            workflow: "".to_string(),
            exc_id: "".to_string(),
            categories: vec![],
            message: "".to_string(),
            continue_url: Some("https://continue.com".to_string()),
            abort_url: Some("https://abort.com".to_string()),
        };

        let expected_output = vec![
            json!({"textButton": {"text": "Continue","onClick": {"openLink": {"url": "https://continue.com"}}}}),
            json!({"textButton": {"text": "Abort","onClick": {"openLink": {"url": "https://abort.com"}}}}),
        ];

        let output = create_card_buttons(&event);
        assert_eq!(output, expected_output);
    }
}

pub fn create_card_message(event: &AMErrorEvent) -> Value {
    let call_out = format_recepients(&event.categories, "#");
    let message = create_header_message(event);
    let buttons = create_card_buttons(event);

    json!({
        "text": call_out,
        "cards": [{
            "header": {
                "title": "Alert!",
                "imageUrl": "https://developers.google.com/chat/images/quickstart-app-avatar.png"
            },
            "sections": [{
                "widgets": [{
                    "textParagraph": {
                        "text": message,
                    }
                }]
            },{
                "widgets": [{
                    "buttons": buttons
                }]
            }]
        }]
    })
}

#[cfg(test)]
mod tests_create_card_message {
    use super::*;

    #[test]
    fn test_create_card_message_no_urls() {
        let event = AMErrorEvent {
            workflow: "workflow1".to_string(),
            exc_id: "exc_id1".to_string(),
            categories: vec!["admin".to_string()],
            message: "Error message".to_string(),
            continue_url: None,
            abort_url: None,
        };

        let expected_output = json!({
            "cards": [
                {
                    "header": {
                        "title": "Alert!",
                        "imageUrl": "https://developers.google.com/chat/images/quickstart-app-avatar.png"
                    },
                    "sections": [
                        {
                            "widgets": [
                                {
                                    "textParagraph": {
                                        "text": "Hi @admin, The workflow: <b><font color='black'>workflow1</font></b>; \
                                        failed for the production ID: <b><font color='black'>exc_id1</font></b>. \
                                        This is the error message: <font color='#FF0000'>Error message</font>"
                                    }
                                }
                            ]
                        },
                        {
                            "widgets": [
                                {
                                    "buttons": []
                                }
                            ]
                        }
                    ]
                }
            ],
            "text": "#admin"
        }
        );

        let output = create_card_message(&event);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_create_card_message_with_urls() {
        let event = AMErrorEvent {
            workflow: "workflow1".to_string(),
            exc_id: "exc_id1".to_string(),
            categories: vec!["admin".to_string()],
            message: "Error message".to_string(),
            continue_url: Some("https://continue.com".to_string()),
            abort_url: Some("https://abort.com".to_string()),
        };

        let expected_output = json!({
            "cards": [
                {
                    "header": {
                        "imageUrl": "https://developers.google.com/chat/images/quickstart-app-avatar.png",
                        "title": "Alert!"
                    },
                    "sections": [
                        {
                            "widgets": [
                                {
                                    "textParagraph": {
                                        "text": "Hi @admin, The workflow: <b><font color='black'>workflow1</font></b>; failed for the production ID: <b><font color='black'>exc_id1</font></b>. This is the error message: <font color='#FF0000'>Error message</font>"
                                    }
                                }
                            ]
                        },
                        {
                            "widgets": [
                                {
                                    "buttons": [
                                        {
                                            "textButton": {
                                                "onClick": {
                                                    "openLink": {
                                                        "url": "https://continue.com"
                                                    }
                                                },
                                                "text": "Continue"
                                            }
                                        },
                                        {
                                            "textButton": {
                                                "onClick": {
                                                    "openLink": {
                                                        "url": "https://abort.com"
                                                    }
                                                },
                                                "text": "Abort"
                                            }
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            ],
            "text": "#admin"
        }
        );

        let output = create_card_message(&event);
        assert_eq!(output, expected_output);
    }
}
