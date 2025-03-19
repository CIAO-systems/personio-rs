# CompanyEmployeesPostRequestEmployee

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** | The e-mail field is required for the employee creation. Updating of this field is not currently supported. | 
**first_name** | **String** |  | 
**last_name** | **String** |  | 
**preferred_name** | Option<**String**> |  | [optional]
**gender** | Option<**String**> |  | [optional]
**position** | Option<**String**> |  | [optional]
**subcompany** | Option<**String**> | The subcompany employee belongs to. Should be predefined in Personio. Otherwise will be ignored with showing meta error in the response. | [optional]
**department** | Option<**String**> | The department employee belongs to. Should be predefined in Personio. Otherwise will be ignored with showing meta error in the response. | [optional]
**office** | Option<**String**> | The office employee belongs to. Should be predefined in Personio. Otherwise will be ignored with showing meta error in the response. | [optional]
**hire_date** | Option<[**String**](string.md)> | Employee hire date. Format: \"yyyy-mm-dd\". If `status` is not provided, it will be set to `active` if the hire date is in the past, or to `onboarding` if it's in the future. | [optional]
**weekly_working_hours** | Option<**f64**> |  | [optional]
**status** | Option<**String**> | Status of the employee. Overrides the status determined based on the value of `hire_date`. | [optional]
**supervisor_id** | Option<**f64**> | Employee ID of the Supervisor to be assigned. It needs to belong to a current existing employee, otherwise an error will be returned. If not present, no supervisor will be assigned. | [optional]
**custom_attributes** | Option<[**models::CompanyEmployeesPostRequestEmployeeCustomAttributes**](_company_employees_post_request_employee_custom_attributes.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


