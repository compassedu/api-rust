use serde::{Deserialize, Serialize};

use crate::utils::consts::USER_AGENT;
#[deprecated(note = "subject to change")]
pub async fn get_attendance_summary(
    cookies: String,
    start_date: String,
    end_date: String,
    user_id: i32,
    school_id: &str,
) -> Result<Vec<AttendanceSummary>, reqwest::Error> {
    let url = format!(
        "https://{}.compass.education/Services/Attendance.svc/GetAttendanceSummary",
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
        .json(&GetAttendanceSumReq {
            start_date,
            end_date,
            student_status: "2".to_string(),
            in_class: ["0".to_string(), "1".to_string()].to_vec(),
            ok_class: ["0".to_string(), "1".to_string()].to_vec(),
            vce: ["0".to_string(), "1".to_string()].to_vec(),
            schl: ["0".to_string(), "1".to_string()].to_vec(),
            perspective: "1".to_string(),
            total_whole_day_limit: "0".to_string(),
            total_partial_day_limit: "0".to_string(),
            user_id: user_id.to_string(),
        })
        .send()
        .await?;
    let res = req.json::<GetAttendanceSumRes>().await?;
    let mut attendance_summary: Vec<AttendanceSummary> = Vec::new();
    for subject in res.d {
        let v: AttendanceSummary = AttendanceSummary {
            authorized_excused_absence: subject.authorized_excused_absence,
            attendance_id: subject.attendance_id,
            attendance_name: subject.attendance_name,
            authorized_absence: subject.authorized_absence,
            count: subject.count,
            counted: subject.counted,
            form: subject.form,
            total_out_of_class: subject.total_out_of_class,
            late_explained: subject.late_unexplained,
            last_name: subject.last_name,
            late_unexplained: subject.lu,
            not_present_counted_vce_ok: subject.not_present_counted_vce_ok,
            notcounted: subject.notcounted,
            npa: subject.npa,
            npu: subject.npu,
            present: subject.p,
            in_class_percentage: subject.pa,
            accounted_for_percentage: subject.pok,
            subject_name: subject.subject_name,
            school_percentage: subject.spc,
            total_in_class: subject.ta,
            total_out_of_class_: subject.tna,
            user_id: subject.user_id,
            username: subject.un,
            unauthorizedabsence: subject.unauthorizedabsence,
            vceok: subject.vceok,
            vpc: subject.vpc,
        };
        attendance_summary.push(v);
    }
    Ok(attendance_summary)
}
#[derive(Serialize, Deserialize, Debug)]
struct GetAttendanceSumReq {
    #[serde(rename = "startDate")]
    start_date: String,
    #[serde(rename = "endDate")]
    end_date: String,
    #[serde(rename = "studentStatus")]
    student_status: String,
    #[serde(rename = "inClass")]
    in_class: Vec<String>,
    #[serde(rename = "okClass")]
    ok_class: Vec<String>,
    vce: Vec<String>,
    schl: Vec<String>,
    perspective: String,
    #[serde(rename = "totalWholeDayLimit")]
    total_whole_day_limit: String,
    #[serde(rename = "totalPartialDayLimit")]
    total_partial_day_limit: String,
    #[serde(rename = "userId")]
    user_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct AttendanceSumRes {
    #[serde(rename = "aea")]
    authorized_excused_absence: i32,
    #[serde(rename = "aid")]
    attendance_id: i32,
    #[serde(rename = "an")]
    attendance_name: String,
    #[serde(rename = "authorizedabsence")]
    authorized_absence: i32,
    #[serde(rename = "c")]
    count: i32,
    #[serde(rename = "counted")]
    counted: i32,
    #[serde(rename = "fg")]
    form: String,
    #[serde(rename = "im")]
    total_out_of_class: i32,
    #[serde(rename = "la")]
    late_unexplained: i32,
    #[serde(rename = "ln")]
    last_name: String,
    // #[serde(rename = "lu")]
    lu: i32,
    #[serde(rename = "notPresentCountedVceOk")]
    not_present_counted_vce_ok: i32,
    // #[serde(rename = "ln")]
    notcounted: i32,
    // #[serde(rename = "ln")]
    npa: i32,
    // #[serde(rename = "ln")]
    npu: i32,
    // #[serde(rename = "ln")]
    p: i32,
    // #[serde(rename = "ln")]
    pa: String,
    // #[serde(rename = "ln")]
    pok: String,
    #[serde(rename = "sn")]
    subject_name: String,
    // #[serde(rename = "ln")]
    spc: String,
    // #[serde(rename = "ln")]
    ta: i32,
    // #[serde(rename = "ln")]
    tna: i32,
    #[serde(rename = "uid")]
    user_id: i32,
    // #[serde(rename = "ln")]
    un: String,
    // #[serde(rename = "ln")]
    unauthorizedabsence: i32,
    // #[serde(rename = "ln")]
    vceok: i32,
    // #[serde(rename = "ln")]
    vpc: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AttendanceSummary {
    pub authorized_excused_absence: i32,
    pub attendance_id: i32,
    pub attendance_name: String,
    pub authorized_absence: i32,
    pub count: i32,
    pub counted: i32,
    pub form: String,
    pub total_out_of_class: i32,
    /// # Late: Explained
    /// Number of times the student was late explained (parent or school), for the given class.
    pub late_explained: i32,
    pub last_name: String,
    /// # Late: Unexplained
    /// Number of times the student was late for the given class, and there is no explanation (parent or school) in the system.
    pub late_unexplained: i32,
    pub not_present_counted_vce_ok: i32,
    pub notcounted: i32,
    pub npa: i32,
    pub npu: i32,
    /// # Present
    /// Number of times the student was marked present for the given class.
    pub present: i32,
    /// # In Class' Percentage
    /// For the time allocated to the given class, what percentage was the student doing that actual class/subject (and not something else).
    ///
    /// ## Calculation
    /// Present + Late / Sessions Run
    ///
    /// This figure is most useful for individual subject/class teachers
    pub in_class_percentage: String,
    /// # Accounted For Percentage
    /// For the time allocated to the given class, what percentage of this time was the student actually accounted for?
    ///
    /// ## Calculation
    /// Present + Late + Non-Counted + Counted / Sessions Run
    pub accounted_for_percentage: String,
    pub subject_name: String,
    /// # School Percentage
    /// For the time allocated to the given class, what percentage of this time was the student's whereabouts explained/accounted for by the school?
    ///
    /// ## Calculation
    /// Present + Late + Non-Counted / Sessions Run
    pub school_percentage: String,
    /// # Total In Class
    pub total_in_class: i32,
    pub total_out_of_class_: i32,
    pub user_id: i32,
    pub username: String,
    pub unauthorizedabsence: i32,
    pub vceok: i32,
    pub vpc: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct GetAttendanceSumRes {
    d: Vec<AttendanceSumRes>,
}
pub async fn get_half_day_summary_grid_lines(
    cookies: String,
    year: i32,
    user_id: i32,
    school_id: &str,
) -> Result<Vec<SummaryGridLine>, reqwest::Error> {
    let url = format!(
        "https://{}.compass.education/Services/AttendanceV2.svc/GetHalfDaySummaryGridLines",
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
        .json(&GetHalfDaySummaryGridLinesReq {
            user_id,
            year,
            page: 1,
            start: 0,
            limit: 2500,
        })
        .send()
        .await?;
    let res = req.json::<GetHalfDaySummaryGridLinesRes>().await?;
    let mut get_half_day_summary_grid_lines: Vec<SummaryGridLine> = Vec::new();
    for i in res.d {
        let v = SummaryGridLine {
            am_extended_status_id: i.am_extended_status_id,
            am_status: i.am_status,
            am_status_desc: i.am_status_desc,
            am_status_export_identifier: i.am_status_export_identifier,
            am_status_period_calc_not_marked: i.am_status_period_calc_not_marked,
            counted_absence: i.counted_absence,
            counted_absence_export_identifier: i.counted_absence_export_identifier,
            counted_absence_reason_desc: i.counted_absence_reason_desc,
            counted_absence_reason_status: i.counted_absence_reason_status,
            date: i.date,
            date_equality_property: i.date_equality_property,
            day_of_week: i.day_of_week,
            most_prevalent_desc: i.most_prevalent_desc,
            most_prevalent_export_identifier: i.most_prevalent_export_identifier,
            most_prevalent_status: i.most_prevalent_status,
            partial_absence: i.partial_absence,
            partial_day_export_identifier: i.partial_day_export_identifier,
            pm_extended_status_id: i.pm_extended_status_id,
            pm_status: i.pm_status,
            pm_status_desc: i.pm_status_desc,
            pm_status_export_identifier: i.pm_status_export_identifier,
            pm_status_period_calc_not_marked: i.pm_status_period_calc_not_marked,
            qld_half_day_code_am: i.qld_half_day_code_am,
            qld_half_day_code_pm: i.qld_half_day_code_pm,
            time_amount_absent: i.time_amount_absent,
            time_amount_absent_counted: i.time_amount_absent_counted,
            time_amount_absent_not_counted: i.time_amount_absent_not_counted,
            time_amount_arrived_late_am: i.time_amount_arrived_late_am,
            time_amount_arrived_late_pm: i.time_amount_arrived_late_pm,
            time_amount_expected: i.time_amount_expected,
            time_amount_not_marked: i.time_amount_not_marked,
            time_amount_unscheduled: i.time_amount_unscheduled,
            user_id,
            whole_day_absence: i.whole_day_absence,
            whole_day_export_identifier: i.whole_day_export_identifier,
        };
        get_half_day_summary_grid_lines.push(v)
    }
    Ok(get_half_day_summary_grid_lines)
}
#[derive(Serialize, Deserialize, Debug)]
struct GetHalfDaySummaryGridLinesReq {
    #[serde(rename = "userId")]
    user_id: i32,
    year: i32,
    page: i32,
    start: i32,
    limit: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct GetHalfDaySummaryGridLinesRes {
    d: Vec<SummaryGridLineRes>,
}
#[derive(Serialize, Deserialize, Debug)]
struct SummaryGridLineRes {
    #[serde(rename = "amExtendedStatusId")]
    am_extended_status_id: i64,
    #[serde(rename = "amStatus")]
    am_status: i16,
    #[serde(rename = "amStatusDesc")]
    am_status_desc: String,
    #[serde(rename = "amStatusExportIdentifier")]
    am_status_export_identifier: String,
    #[serde(rename = "amStatusPeriodCalcNotMarked")]
    am_status_period_calc_not_marked: bool,
    #[serde(rename = "countedAbsence")]
    counted_absence: bool,
    #[serde(rename = "countedAbsenceExportIdentifier")]
    counted_absence_export_identifier: Option<String>,
    #[serde(rename = "countedAbsenceReasonDesc")]
    counted_absence_reason_desc: String,
    #[serde(rename = "countedAbsenceReasonStatus")]
    counted_absence_reason_status: Option<i32>,
    date: String,
    #[serde(rename = "dateEqualityProperty")]
    date_equality_property: String,
    #[serde(rename = "dayOfWeek")]
    day_of_week: i8,
    #[serde(rename = "mostPrevalentDesc")]
    most_prevalent_desc: String,
    #[serde(rename = "mostPrevalentExportIdentifier")]
    most_prevalent_export_identifier: Option<String>,
    #[serde(rename = "mostPrevalentStatus")]
    most_prevalent_status: Option<i16>,
    #[serde(rename = "partialAbsence")]
    partial_absence: bool,
    #[serde(rename = "partialDayExportIdentifier")]
    partial_day_export_identifier: String,
    #[serde(rename = "pmExtendedStatusId")]
    pm_extended_status_id: i32,
    #[serde(rename = "pmStatus")]
    pm_status: i16,
    #[serde(rename = "pmStatusDesc")]
    pm_status_desc: String,
    #[serde(rename = "pmStatusExportIdentifier")]
    pm_status_export_identifier: String,
    #[serde(rename = "pmStatusPeriodCalcNotMarked")]
    pm_status_period_calc_not_marked: bool,
    #[serde(rename = "qldHalfDayCodeAM")]
    qld_half_day_code_am: i8,
    #[serde(rename = "qldHalfDayCodePM")]
    qld_half_day_code_pm: i8,
    #[serde(rename = "timeAmountAbsent")]
    time_amount_absent: i64,
    #[serde(rename = "timeAmountAbsentCounted")]
    time_amount_absent_counted: i64,
    #[serde(rename = "timeAmountAbsentNotCounted")]
    time_amount_absent_not_counted: i64,
    #[serde(rename = "timeAmountArrivedLateAm")]
    time_amount_arrived_late_am: Option<String>,
    #[serde(rename = "timeAmountArrivedLatePm")]
    time_amount_arrived_late_pm: Option<String>,
    #[serde(rename = "timeAmountExpected")]
    time_amount_expected: i64,
    #[serde(rename = "timeAmountNotMarked")]
    time_amount_not_marked: i64,
    #[serde(rename = "timeAmountUnscheduled")]
    time_amount_unscheduled: i64,
    #[serde(rename = "userId")]
    user_id: i32,
    #[serde(rename = "wholeDayAbsence")]
    whole_day_absence: bool,
    #[serde(rename = "wholeDayExportIdentifier")]
    whole_day_export_identifier: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryGridLine {
    pub am_extended_status_id: i64,
    pub am_status: i16,
    pub am_status_desc: String,
    pub am_status_export_identifier: String,
    pub am_status_period_calc_not_marked: bool,
    pub counted_absence: bool,
    pub counted_absence_export_identifier: Option<String>,
    pub counted_absence_reason_desc: String,
    pub counted_absence_reason_status: Option<i32>,
    pub date: String,
    pub date_equality_property: String,
    pub day_of_week: i8,
    pub most_prevalent_desc: String,
    pub most_prevalent_export_identifier: Option<String>,
    pub most_prevalent_status: Option<i16>,
    pub partial_absence: bool,
    pub partial_day_export_identifier: String,
    pub pm_extended_status_id: i32,
    pub pm_status: i16,
    pub pm_status_desc: String,
    pub pm_status_export_identifier: String,
    pub pm_status_period_calc_not_marked: bool,
    pub qld_half_day_code_am: i8,
    pub qld_half_day_code_pm: i8,
    pub time_amount_absent: i64,
    pub time_amount_absent_counted: i64,
    pub time_amount_absent_not_counted: i64,
    pub time_amount_arrived_late_am: Option<String>,
    pub time_amount_arrived_late_pm: Option<String>,
    pub time_amount_expected: i64,
    pub time_amount_not_marked: i64,
    pub time_amount_unscheduled: i64,
    pub user_id: i32,
    pub whole_day_absence: bool,
    pub whole_day_export_identifier: String,
}
// pub async fn get_periods_for_timeline(
//     cookies: String,
//     date: String,
//     user_id: i32,
//     school_id: &str,
// ) {
// }
// pub async fn get_half_day_summary_grid_lines(
//     cookies: String,
//     year: i32,
//     user_id: i32,
//     school_id: &str,
// ) {
// }
