# Absence

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**start_date** | Option<**String**> |  | [optional]
**end_date** | Option<**String**> |  | [optional]
**days_count** | Option<**f64**> |  | [optional]
**half_day_start** | Option<**bool**> |  | [optional]
**half_day_end** | Option<**bool**> |  | [optional]
**time_off_type** | Option<[**models::AbsenceTimeOffType**](Absence_time_off_type.md)> |  | [optional]
**employee** | Option<[**models::ShortEmployee**](ShortEmployee.md)> |  | [optional]
**certificate** | Option<[**models::HourlyAbsencePeriodResponseAttributesCertificate**](HourlyAbsencePeriodResponseAttributes_certificate.md)> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**created_by** | Option<**String**> | API if the origin is api, otherwise returns an admin employee's name who's account is used to create the absence | [optional]
**updated_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


