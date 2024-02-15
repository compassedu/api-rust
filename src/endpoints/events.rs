use serde::{Deserialize, Serialize};

use crate::utils::consts::USER_AGENT;
/// Retrieves upcoming events.
///
/// # Arguments
///
/// * `cookies` - Cookies for authentication.
/// * `school_id` - The ID of the school.
pub async fn get_upcoming_events(
    cookies: String,
    school_id: &str,
) -> Result<Vec<Event>, reqwest::Error> {
    let url = format!(
        "https://{}.compass.education/Services/ActionCentre.svc/GetEvents",
        school_id
    );
    let mut headers = reqwest::header::HeaderMap::new();
    let cookies_str: &str = cookies.as_str();
    headers.insert(
        "cookie",
        reqwest::header::HeaderValue::from_str(&cookies_str).unwrap(),
    );
    let req = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .default_headers(headers)
        .user_agent(USER_AGENT)
        .build()?
        .post(url)
        .json("{}")
        .send()
        .await?;
    let res = req.json::<GetUpcomingEventsRes>().await?;
    let mut events: Vec<Event> = Vec::new();
    for event in res.d {
        let mut sessions: Vec<Session> = Vec::new();
        for session in event.sessions {
            let e = Session {
                campus_name: session.campus_name,
                finish: session.finish,
                instance_id: session.instance_id,
                location_comments: session.location_comments,
                start: session.start,
            };
            sessions.push(e);
        }
        let e = Event {
            id: event.id,
            name: event.name,
            additional_contact_details: event.additional_contact_details,
            additional_details: event.additional_details,
            administration_details: event.administration_details,
            allow_consent_without_payment: event.allow_consent_without_payment,
            allow_decline: event.allow_decline,
            amount_paid: event.amount_paid,
            attendee_status: event.attendee_status,
            confirmed_attendees_count: event.confirmed_attendees_count,
            consent_date: event.consent_dt,
            consent_form_id: event.consent_form_id,
            consent_name: event.consent_name,
            consent_payment_due: event.consent_payment_due,
            consent_return_location: event.consent_return_location,
            cost: event.cost,
            description: event.description,
            dress_code: event.dress_code,
            educative_purpose: event.educative_purpose,
            finish: event.finish,
            is_opt_in: event.is_opt_in,
            location: event.location,
            start: event.start,
            student_id: event.student_id,
            student_consent_content: event.student_consent_content,
            sessions,
            transport: event.transport,
        };
        events.push(e)
    }
    Ok(events)
}
/// Retrieves past events.
///
/// # Arguments
///
/// * `cookies` - Cookies for authentication.
/// * `school_id` - The ID of the school.
pub async fn get_past_events(
    cookies: String,
    school_id: &str,
) -> Result<Vec<Event>, reqwest::Error> {
    let url = format!(
        "https://{}.compass.education/Services/ActionCentre.svc/GetPastEvents",
        school_id
    );
    let mut headers = reqwest::header::HeaderMap::new();
    let cookies_str: &str = cookies.as_str();
    headers.insert(
        "cookie",
        reqwest::header::HeaderValue::from_str(&cookies_str).unwrap(),
    );
    let req = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .default_headers(headers)
        .user_agent(USER_AGENT)
        .build()?
        .post(url)
        .json("{}")
        .send()
        .await?;
    let res = req.json::<GetUpcomingEventsRes>().await?;
    let mut events: Vec<Event> = Vec::new();
    for event in res.d {
        let mut sessions: Vec<Session> = Vec::new();
        for session in event.sessions {
            let e = Session {
                campus_name: session.campus_name,
                finish: session.finish,
                instance_id: session.instance_id,
                location_comments: session.location_comments,
                start: session.start,
            };
            sessions.push(e);
        }
        let e = Event {
            id: event.id,
            name: event.name,
            additional_contact_details: event.additional_contact_details,
            additional_details: event.additional_details,
            administration_details: event.administration_details,
            allow_consent_without_payment: event.allow_consent_without_payment,
            allow_decline: event.allow_decline,
            amount_paid: event.amount_paid,
            attendee_status: event.attendee_status,
            confirmed_attendees_count: event.confirmed_attendees_count,
            consent_date: event.consent_dt,
            consent_form_id: event.consent_form_id,
            consent_name: event.consent_name,
            consent_payment_due: event.consent_payment_due,
            consent_return_location: event.consent_return_location,
            cost: event.cost,
            description: event.description,
            dress_code: event.dress_code,
            educative_purpose: event.educative_purpose,
            finish: event.finish,
            is_opt_in: event.is_opt_in,
            location: event.location,
            start: event.start,
            student_id: event.student_id,
            student_consent_content: event.student_consent_content,
            sessions,
            transport: event.transport,
        };
        events.push(e)
    }
    Ok(events)
}
/// Retrieves upcoming & past events.
///
/// # Arguments
///
/// * `cookies` - Cookies for authentication.
/// * `school_id` - The ID of the school.
pub async fn get_all_events(
    cookies: String,
    school_id: &str,
) -> Result<Vec<Event>, reqwest::Error> {
    let past_events = get_past_events(cookies.clone(), school_id).await?;
    let upcoming_events = get_upcoming_events(cookies, school_id).await?;
    let mut all_events = Vec::new();
    all_events.extend(past_events);
    all_events.extend(upcoming_events);
    Ok(all_events)
}
#[derive(Serialize, Deserialize, Debug)]
struct GetUpcomingEventsRes {
    d: Vec<ActionCentreEvent>,
}
#[derive(Serialize, Deserialize, Debug)]
struct ActionCentreEvent {
    id: i32,
    name: String,
    #[serde(rename = "additionalContactDetails")]
    additional_contact_details: Option<String>,
    #[serde(rename = "additionalDetails")]
    additional_details: String,
    #[serde(rename = "administrationDetails")]
    administration_details: String,
    #[serde(rename = "allowConsentWithoutPayment")]
    allow_consent_without_payment: bool,
    #[serde(rename = "allowDecline")]
    allow_decline: bool,
    #[serde(rename = "amountPaid")]
    amount_paid: f32,
    #[serde(rename = "attendeeStatus")]
    attendee_status: i32,
    #[serde(rename = "confirmedAttendeesCount")]
    confirmed_attendees_count: i32,
    #[serde(rename = "consentDt")]
    consent_dt: Option<String>,
    #[serde(rename = "consentFormId")]
    consent_form_id: Option<i32>,
    #[serde(rename = "consentName")]
    consent_name: Option<String>,
    #[serde(rename = "consentPaymentDue")]
    consent_payment_due: String,
    #[serde(rename = "consentReturnLocation")]
    consent_return_location: Option<String>,
    cost: f32,
    description: Option<String>,
    #[serde(rename = "dressCode")]
    dress_code: String,
    #[serde(rename = "educativePurpose")]
    educative_purpose: String,
    finish: String,
    #[serde(rename = "isOptIn")]
    is_opt_in: bool,
    location: Option<String>,
    start: String,
    #[serde(rename = "studentId")]
    student_id: i32,
    #[serde(rename = "studentConsentContent")]
    student_consent_content: String,
    sessions: Vec<ActionCentreEventSession>,
    transport: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct ActionCentreEventSession {
    // location: Option<String>,
    #[serde(rename = "campusName")]
    campus_name: String,
    finish: String,
    #[serde(rename = "instanceId")]
    instance_id: String,
    #[serde(rename = "locationComments")]
    location_comments: String,
    start: String,
}
/// Represents an event.
#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    /// The ID of the event.
    pub id: i32,
    /// The name of the event.
    pub name: String,
    /// Additional contact details for the event.
    pub additional_contact_details: Option<String>,
    /// Additional details of the event.
    pub additional_details: String,
    /// Administration details of the event.
    pub administration_details: String,
    /// Indicates whether consent can be given without payment.
    pub allow_consent_without_payment: bool,
    /// Indicates whether decline is allowed for the event.
    pub allow_decline: bool,
    /// The amount paid for the event.
    pub amount_paid: f32,
    /// The status of the attendee.
    pub attendee_status: i32,
    /// The number of confirmed attendees.
    pub confirmed_attendees_count: i32,
    /// The date of consent.
    pub consent_date: Option<String>,
    /// The ID of the consent form.
    pub consent_form_id: Option<i32>,
    /// The name of the consent form.
    pub consent_name: Option<String>,
    /// The due date for consent payment.
    pub consent_payment_due: String,
    /// The return location for consent.
    pub consent_return_location: Option<String>,
    /// The cost of the event.
    pub cost: f32,
    /// The description of the event.
    pub description: Option<String>,
    /// The dress code for the event.
    pub dress_code: String,
    /// The educative purpose of the event.
    pub educative_purpose: String,
    /// The finish time of the event.
    pub finish: String,
    /// Indicates whether the event is opt-in.
    pub is_opt_in: bool,
    /// The location of the event.
    pub location: Option<String>,
    /// The start time of the event.
    pub start: String,
    /// The ID of the student associated with the event.
    pub student_id: i32,
    /// The consent content for the student.
    pub student_consent_content: String,
    /// The sessions associated with the event.
    pub sessions: Vec<Session>,
    /// The transport information for the event.
    pub transport: String,
}
/// Represents a session within an event.
#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    // location: Option<String>,
    /// The name of the campus where the session takes place.
    pub campus_name: String,
    /// The finish time of the session.
    pub finish: String,
    /// The instance ID of the session.
    pub instance_id: String,
    /// Comments regarding the location of the session.
    pub location_comments: String,
    /// The start time of the session.
    pub start: String,
}
// enum AttendeeStatus {
//     ATTENDING,
//     NA,
//     OTHER,
// }
