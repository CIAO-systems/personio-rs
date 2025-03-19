# CompanyEmployeesEmployeeIdPatchRequestEmployee

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**first_name** | Option<**String**> |  | [optional]
**last_name** | Option<**String**> |  | [optional]
**preferred_name** | Option<**String**> |  | [optional]
**gender** | Option<**String**> |  | [optional]
**position** | Option<**String**> |  | [optional]
**subcompany** | Option<**String**> | The subcompany employee belongs to. Should be predefined in Personio. Otherwise will be ignored with showing meta error in the response. | [optional]
**department** | Option<**String**> | The department employee belongs to. Should be predefined in Personio. Otherwise will be ignored with showing meta error in the response. | [optional]
**office** | Option<**String**> | The office employee belongs to. Should be predefined in Personio. Otherwise will be ignored with showing meta error in the response. | [optional]
**hire_date** | Option<[**String**](string.md)> | Employee hire date. Format: \"yyyy-mm-dd\". Update of the `hire_date` will not update employee status on its own (for that you'll need to update the `status` field) | [optional]
**weekly_working_hours** | Option<**f64**> |  | [optional]
**status** | Option<**String**> | Status of the employee. | [optional]
**supervisor_id** | Option<**f64**> | Employee ID of the Supervisor to be assigned. It needs to belong to a current existing employee and not the same as the one of the employee being updated, otherwise an error will be returned. If sent as null, will unset the employee's supervisor. | [optional]
**custom_attributes** | Option<[**models::CompanyEmployeesPostRequestEmployeeCustomAttributes**](_company_employees_post_request_employee_custom_attributes.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


