use serde::{Deserialize, Serialize};

use crate::utils::consts::USER_AGENT;

/// Retrieves calendar events for a specific user within a given time range.
///
/// # Arguments
///
/// * `cookies` - Cookies for authentication.
/// * `user_id` - ID of the user whose calendar events are to be retrieved.
/// * `start` - Start date for the calendar events (in a string format).
/// * `end` - End date for the calendar events (in a string format).
/// * `school_id` - The ID of the school.
/// # Example
///
/// ```
/// let events = get_calendar_events_by_user("***".to_string(), 0, "2024-05-08".to_string(), "2024-05-08".to_string(), "***").await?;
/// ```
pub async fn get_calendar_events_by_user(
    cookies: String,
    user_id: i32,
    start: String,
    end: String,
    school_id: &str,
) -> Result<Vec<Event>, reqwest::Error> {
    let url = format!(
        "https://{}.compass.education/Services/Calendar.svc/GetCalendarEventsByUser",
        school_id
    );
    let mut headers = reqwest::header::HeaderMap::new();
    let cookies_str: &str = cookies.as_str();
    headers.insert(
        "cookie",
        reqwest::header::HeaderValue::from_str(&cookies_str).unwrap(),
    );
    let body = GetCalendarEventsByUserRequest {
        user_id: user_id,
        start_date: start,
        end_date: end,
        limit: 50,
        start: 0,
        page: 1,
    };
    let req = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .default_headers(headers)
        .user_agent(USER_AGENT)
        .build()?
        .post(url)
        .json(&body)
        .send()
        .await?;
    let res = req.json::<GetCalendarEventsByUserResponse>().await?;
    let mut events: Vec<Event> = Vec::new();
    for event in res.d {
        let mut locations: Vec<EventLocation> = Vec::new();
        let mut managers: Vec<EventManager> = Vec::new();
        for location in event.locations {
            let t = EventLocation {
                covering_location_id: location.covering_location_id,
                covering_location_name: location.covering_location_name,
                location_id: location.location_id,
                location_name: location.location_name,
            };
            locations.push(t)
        }
        for manager in event.managers {
            let t = EventManager {
                covering_import_identifier: manager.covering_import_identifier,
                covering_user_id: manager.covering_user_id,
                manager_import_identifier: manager.manager_import_identifier,
                manager_user_id: manager.manager_user_id,
            };
            managers.push(t)
        }
        let e = Event {
            activity_id: event.activity_id,
            activity_import_identifier: event.activity_import_identifier,
            activity_type: event.activity_type,
            all_day: event.all_day,
            attendance_mode: event.attendance_mode,
            attendee_user_id: event.attendee_user_id,
            background_color: event.background_color,
            calendar_id: event.calendar_id,
            category_ids: event.category_ids,
            comment: event.comment,
            description: event.description,
            event_setup_status: event.event_setup_status,
            finish: event.finish,
            guid: event.guid,
            in_class_status: event.in_class_status,
            instance_id: event.instance_id,
            is_recurring: event.is_recurring,
            learning_task_id: event.learning_task_id,
            lesson_plan_configured: event.lesson_plan_configured,
            location: event.location,
            long_title: event.long_title,
            long_title_without_time: event.long_title_without_time,
            manager_id: event.manager_id,
            minutes_meeting_id: event.minutes_meeting_id,
            period: event.period,
            recurring_finish: event.recurring_finish,
            recurring_start: event.recurring_start,
            repeat_days: event.repeat_days,
            repeat_forever: event.repeat_forever,
            repeat_frequency: event.repeat_frequency,
            repeat_until: event.repeat_until,
            roll_marked: event.roll_marked,
            running_status: event.running_status,
            start: event.start,
            target_student_id: event.target_student_id,
            teaching_days_only: event.teaching_days_only,
            text_color: event.text_color,
            title: event.title,
            unavailable_pd: event.unavailable_pd,
            locations: locations,
            managers: managers,
        };
        events.push(e)
    }
    Ok(events)
}
#[derive(Serialize, Deserialize, Debug)]
struct GetCalendarEventsByUserResponse {
    d: Vec<CalendarEvent>,
}
#[derive(Serialize, Deserialize, Debug)]
struct GetCalendarEventsByUserRequest {
    #[serde(rename = "userId")]
    user_id: i32,
    #[serde(rename = "startDate")]
    start_date: String,
    #[serde(rename = "endDate")]
    end_date: String,
    limit: i32,
    start: i32,
    page: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct CalendarEvent {
    #[serde(rename = "activityId")]
    activity_id: i32,
    #[serde(rename = "activityImportIdentifier")]
    activity_import_identifier: Option<String>,
    #[serde(rename = "activityType")]
    activity_type: i32,
    #[serde(rename = "allDay")]
    all_day: bool,
    #[serde(rename = "attendanceMode")]
    attendance_mode: i32,
    #[serde(rename = "attendeeUserId")]
    attendee_user_id: i32,
    #[serde(rename = "backgroundColor")]
    background_color: String,
    #[serde(rename = "calendarId")]
    calendar_id: Option<i32>,
    #[serde(rename = "categoryIds")]
    category_ids: Option<Vec<i32>>,
    comment: Option<String>,
    description: String,
    #[serde(rename = "eventSetupStatus")]
    event_setup_status: Option<i32>,
    finish: String,
    guid: String,
    #[serde(rename = "inClassStatus")]
    in_class_status: Option<i32>,
    #[serde(rename = "instanceId")]
    instance_id: String,
    #[serde(rename = "isRecurring")]
    is_recurring: bool,
    #[serde(rename = "learningTaskId")]
    learning_task_id: Option<i32>,
    #[serde(rename = "lessonPlanConfigured")]
    lesson_plan_configured: bool,
    location: Option<i32>,
    locations: Vec<CalendarEventLocation>,
    #[serde(rename = "longTitle")]
    long_title: String,
    #[serde(rename = "longTitleWithoutTime")]
    long_title_without_time: String,
    #[serde(rename = "managerId")]
    manager_id: i32,
    managers: Vec<CalendarEventManager>,
    #[serde(rename = "minutesMeetingId")]
    minutes_meeting_id: Option<i32>,
    period: String,
    #[serde(rename = "recurringFinish")]
    recurring_finish: Option<String>,
    #[serde(rename = "recurringStart")]
    recurring_start: Option<String>,
    #[serde(rename = "repeatDays")]
    repeat_days: Option<i8>,
    #[serde(rename = "repeatForever")]
    repeat_forever: bool,
    #[serde(rename = "repeatFrequency")]
    repeat_frequency: i32,
    #[serde(rename = "repeatUntil")]
    repeat_until: Option<String>,
    #[serde(rename = "rollMarked")]
    roll_marked: bool,
    #[serde(rename = "runningStatus")]
    running_status: i32,
    start: String,
    #[serde(rename = "targetStudentId")]
    target_student_id: i32,
    #[serde(rename = "teachingDaysOnly")]
    teaching_days_only: bool,
    #[serde(rename = "textColor")]
    text_color: String,
    title: String,
    #[serde(rename = "unavailablePd")]
    unavailable_pd: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct CalendarEventLocation {
    #[serde(rename = "coveringLocationId")]
    covering_location_id: Option<i32>,
    #[serde(rename = "coveringLocationName")]
    covering_location_name: Option<String>,
    #[serde(rename = "locationId")]
    location_id: i32,
    #[serde(rename = "locationName")]
    location_name: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct CalendarEventManager {
    #[serde(rename = "coveringImportIdentifier")]
    covering_import_identifier: Option<String>,
    #[serde(rename = "coveringUserId")]
    covering_user_id: Option<i32>,
    #[serde(rename = "managerImportIdentifier")]
    manager_import_identifier: String,
    #[serde(rename = "managerUserId")]
    manager_user_id: i32,
}

/// Represents an event in the calendar.
#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    /// ID of the activity.
    pub activity_id: i32,
    /// Import identifier for the activity.
    pub activity_import_identifier: Option<String>,
    /// Type of activity.
    pub activity_type: i32,
    /// Indicates if the event is an all-day event.
    pub all_day: bool,
    /// Mode of attendance for the event.
    pub attendance_mode: i32,
    /// ID of the attendee user.
    pub attendee_user_id: i32,
    /// Background color of the event.
    pub background_color: String,
    /// Optional ID of the calendar.
    pub calendar_id: Option<i32>,
    /// Optional category IDs for the event.
    pub category_ids: Option<Vec<i32>>,
    /// Comment for the event.
    pub comment: Option<String>,
    /// Description of the event.
    pub description: String,
    /// Optional setup status for the event.
    pub event_setup_status: Option<i32>,
    /// Finish time of the event. In ISO 8601 format.
    pub finish: String,
    /// Globally unique identifier for the event.
    pub guid: String,
    /// In-class status for the event.
    pub in_class_status: Option<i32>,
    /// Instance ID of the event.
    pub instance_id: String,
    /// Indicates if the event is recurring.
    pub is_recurring: bool,
    /// ID of the learning task.
    pub learning_task_id: Option<i32>,
    /// Indicates if lesson plan is configured for the event.
    pub lesson_plan_configured: bool,
    /// Location ID for the event.
    pub location: Option<i32>,
    /// Locations associated with the event.
    pub locations: Vec<EventLocation>,
    /// Long title of the event.
    /// # Return Sample
    /// `{TIME}: {CLASS IMPORT IDENTIFIER} - {LOCATION NAME} - {MANAGER IMPORT IDENTIFIER}`
    pub long_title: String,
    /// Long title of the event without time.
    /// # Return Sample
    /// `{CLASS IMPORT IDENTIFIER} - {LOCATION NAME} - {MANAGER IMPORT IDENTIFIER}`
    pub long_title_without_time: String,
    /// ID of the manager.
    pub manager_id: i32,
    /// Managers associated with the event.
    pub managers: Vec<EventManager>,
    /// Optional ID of the minutes meeting.
    pub minutes_meeting_id: Option<i32>,
    /// Period of the event.
    pub period: String,
    /// Optional finish date for recurring events.
    pub recurring_finish: Option<String>,
    /// Optional start date for recurring events.
    pub recurring_start: Option<String>,
    /// Optional number of repeat days for recurring events.
    pub repeat_days: Option<i8>,
    /// Indicates if the event repeats forever.
    pub repeat_forever: bool,
    /// Frequency of event repetition.
    pub repeat_frequency: i32,
    /// Optional end date for repeating events.
    pub repeat_until: Option<String>,
    /// Indicates if roll is marked for the event.
    pub roll_marked: bool,
    /// Running status of the event.
    pub running_status: i32,
    /// Start time of the event. In ISO 8601 format.
    pub start: String,
    /// ID of the target student.
    pub target_student_id: i32,
    /// Indicates if teaching days only are considered for the event.
    pub teaching_days_only: bool,
    /// Text color of the event.
    pub text_color: String,
    /// Title of the event.
    pub title: String,
    /// Optional unavailable PD for the event.
    pub unavailable_pd: Option<String>,
}
/// Represents the location of an event.
#[derive(Serialize, Deserialize, Debug)]
pub struct EventLocation {
    /// Optional covering location ID.
    ///
    /// When the ID is filled, it indicates that the room has been changed.
    pub covering_location_id: Option<i32>,
    /// Optional covering location name.
    ///
    /// It indicates that the room has been changed.
    pub covering_location_name: Option<String>,
    /// ID of the location.
    pub location_id: i32,
    /// Name of the location.
    pub location_name: String,
}
/// Represents a manager of an event.
#[derive(Serialize, Deserialize, Debug)]
pub struct EventManager {
    /// Optional covering import identifier.
    ///
    /// It indicates that the manager has been changed.
    pub covering_import_identifier: Option<String>,
    /// Optional covering user ID.
    ///
    /// It indicates that the manager has been changed.
    pub covering_user_id: Option<i32>,
    /// Import identifier of the manager.
    pub manager_import_identifier: String,
    /// ID of the manager user.
    pub manager_user_id: i32,
}
// pub enum ActivityType {
//     /// Represents a class activity.
//     Class,
//     /// Represents an event activity.
//     Event,
// }
