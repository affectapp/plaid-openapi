# Mfa

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **String** | Possible values are `device`, `selections`, or `questions`.  If value is `device`, the MFA answer is `1234`.  If value is `selections`, the MFA answer is always the first option.  If value is `questions`, the MFA answer is  `answer_<i>_<j>` for the j-th question in the i-th round, starting from 0. For example, the answer to the first question in the second round is `answer_1_0`. | 
**question_rounds** | **f32** | Number of rounds of questions. Required if value of `type` is `questions`.  | 
**questions_per_round** | **f32** | Number of questions per round. Required if value of `type` is `questions`. If value of type is `selections`, default value is 2. | 
**selection_rounds** | **f32** | Number of rounds of selections, used if `type` is `selections`. Defaults to 1. | 
**selections_per_question** | **f32** | Number of available answers per question, used if `type` is `selection`. Defaults to 2.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


