# EmployeeAbsenceBalanceDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**category** | Option<**String**> |  | [optional]
**balance** | Option<**f64**> | Effective balance, as of request date; not deducting any upcoming absence periods, but only what has been taken. If the absence period is currently ongoing, the effective balance only considers the part of the period until today.  | [optional]
**available_balance** | Option<**f64**> | Available balance is what actually left to be planned (where upcoming absence periods are already deducted). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


