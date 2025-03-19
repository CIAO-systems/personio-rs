# HourlyAbsencePeriodResponseAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**measurement_unit** | Option<**String**> |  | [optional]
**effective_duration** | **i32** | Period effective duration in minutes | 
**employee** | [**models::ShortEmployee**](ShortEmployee.md) |  | 
**absence_type_id** | [**models::AbsenceType**](AbsenceType.md) |  | 
**certificate** | [**models::HourlyAbsencePeriodResponseAttributesCertificate**](HourlyAbsencePeriodResponseAttributes_certificate.md) |  | 
**start** | **String** |  | 
**end** | Option<**String**> |  | [optional]
**half_day_start** | **bool** |  | 
**half_day_end** | **bool** |  | 
**comment** | Option<**String**> |  | [optional]
**origin** | Option<**String**> |  | 
**status** | **String** |  | 
**created_by** | **i32** | ID of the employee who created the absence period. | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**approved_at** | Option<**String**> |  | [optional]
**breakdowns** | [**Vec<models::AbsencePeriodBreakdown>**](AbsencePeriodBreakdown.md) | Breakdowns of effective duration by day of absence. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


