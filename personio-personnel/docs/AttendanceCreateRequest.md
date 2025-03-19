# AttendanceCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attendances** | Option<[**Vec<models::AttendanceCreateRequestAttendancesInner>**](AttendanceCreateRequest_attendances_inner.md)> |  | [optional]
**skip_approval** | Option<**bool**> | Optional, default value is true. If set to false, the approval status of the attendance period will be \"pending\" if an approval rule is set for the attendances type. The respective approval flow will be triggered. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


