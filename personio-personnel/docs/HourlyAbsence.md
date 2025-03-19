# HourlyAbsence

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**measurement_unit** | Option<**String**> |  | [optional]
**effective_duration** | Option<**i32**> | Period effective duration in minutes | [optional]
**employee** | Option<[**models::ShortEmployee**](ShortEmployee.md)> |  | [optional]
**absence_type** | Option<[**models::AbsenceType**](AbsenceType.md)> |  | [optional]
**certificate** | Option<[**models::HourlyAbsencePeriodResponseAttributesCertificate**](HourlyAbsencePeriodResponseAttributes_certificate.md)> |  | [optional]
**start** | Option<**String**> |  | [optional]
**end** | Option<**String**> |  | [optional]
**half_day_start** | Option<**bool**> |  | [optional]
**half_day_end** | Option<**bool**> |  | [optional]
**comment** | Option<**String**> |  | [optional]
**origin** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**timezone** | Option<**String**> |  | [optional]
**created_by** | Option<**i32**> | ID of the employee who created the absence period. | [optional]
**created_at** | Option<**String**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]
**approved_at** | Option<**String**> |  | [optional]
**breakdowns** | Option<[**Vec<models::AbsencePeriodBreakdown>**](AbsencePeriodBreakdown.md)> | Breakdowns of effective duration by day of absence. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


