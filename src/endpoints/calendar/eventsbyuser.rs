use serde::{Deserialize, Serialize};

use crate::utils::consts::USER_AGENT;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    activity_id: i32,
    activity_import_identifier: Option<String>,
    activity_type: i32,
    all_day: bool,
    attendance_mode: i32,
    attendee_user_id: i32,
    background_color: String,
    calendar_id: Option<i32>,
    category_ids: Option<Vec<i32>>,
    comment: Option<String>,
    description: String,
    event_setup_status: Option<i32>,
    finish: String,
    guid: String,
    in_class_status: Option<i32>,
    instance_id: String,
    is_recurring: bool,
    learning_task_id: Option<i32>,
    lesson_plan_configured: bool,
    location: Option<i32>,
    locations: Vec<EventLocation>,
    long_title: String,
    long_title_without_time: String,
    manager_id: i32,
    managers: Vec<EventManager>,
    minutes_meeting_id: Option<i32>,
    period: String,
    recurring_finish: Option<String>,
    recurring_start: Option<String>,
    repeat_days: Option<i8>,
    repeat_forever: bool,
    repeat_frequency: i32,
    repeat_until: Option<String>,
    roll_marked: bool,
    running_status: i32,
    start: String,
    target_student_id: i32,
    teaching_days_only: bool,
    text_color: String,
    title: String,
    unavailable_pd: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct EventLocation {
    covering_location_id: Option<i32>,
    covering_location_name: Option<String>,
    location_id: i32,
    location_name: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct EventManager {
    covering_import_identifier: Option<String>,
    covering_user_id: Option<i32>,
    manager_import_identifier: String,
    manager_user_id: i32,
}
