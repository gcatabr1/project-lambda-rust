<!-- README.md -->
<!--
Author: Gary Cattabriga
Date: 2022.01.29
*** This is a repo for Opportunity Insights App assets related to CDW
*** Specifically data ETL (extracts, transformations and loads) from CDW to the OI App
*** This repo is organized by functional area (ETL) and schema builds
*** 
*** Reference links are enclosed in brackets [ ] instead of parentheses
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->

<!-- Project logo -->
<p align="center">
  <a href="https://bitbucket.remedypartners.com/projects/CA/repos/project-lambda/browse"><img src="images/lambda.png" alt="Project Lambda"></a>
</p>

<!-- Title -->
<div align="center">
  <h1 style="color:blue;" align="center">Project Lambda</h1>
</div>


<!-- Any shields you want to use. Bitbucket is limited in what you can use -->
<p align="center">
  <a href="https://bitbucket.remedypartners.com/projects/CA/repos/gcattabriga/commits">
  <img src="https://img.shields.io/maintenance/yes/2022?style=plastic"
  alt="Commits">
</p>

<!-- The menu links to the various readme sections. Make these whatever works for your project -->      
<p align="center">
  <a href="#about">About</a> •  
  <a href="#test-data">Test Data</a> •  
  <a href="#code">Code</a> •
  <a href="#credits">Credits</a> •
  <a href="#support">Support</a>
</p>


---
<!-- ABOUT --> 
<a name="about"></a>

## About

A repo for **Project Lambda** 

<br/>

<!-- CONTAINER --> 
<a name="test-data"></a>

## Test Data
Test data consists of 11 randomly selected members and associated 2020 claims (based on admission_date)
- 10 members consist of only Facility (I) and Professional (P) claims, 
- 1 member includes I, P and Pharmacy (R) claims 

| member\_id | claimtype | clm\_cnt | line\_cnt |
| :--- | :--- | :--- | :--- |
| member33500 | I | 1 | 8 |
| member33500 | P | 29 | 57 |
| member33500 | R | 62 | 62 |
| member36600 | I | 3 | 30 |
| member36600 | P | 45 | 60 |
| member37800 | P | 5 | 11 |
| member58401 | I | 4 | 26 |
| member58401 | P | 33 | 70 |
| member64200 | I | 5 | 8 |
| member64200 | P | 30 | 76 |
| member66500 | P | 13 | 18 |
| member74200 | P | 6 | 18 |
| member75802 | P | 7 | 12 |
| member78103 | I | 1 | 1 |
| member78103 | P | 3 | 4 |
| member85401 | I | 3 | 10 |
| member85401 | P | 3 | 4 |
| member88101 | I | 3 | 18 |
| member88101 | P | 12 | 13 |


*note:*
- The test data can be found in CDW here: test.pl_sample_claims
- The SQL that produces the JSON can be found [in this repo](https://bitbucket.remedypartners.com/projects/CA/repos/project-lambda/browse/test_data/lambda_project_testdata_json.sql)

<br/>

#### JSON test data files
Three test JSON claims files of various lengths have been created:
1. **lambda_project_testdata_short.json**  -- (member78103, member85401)
2. **lambda_project_testdata_medium.json**  -- (member33500, member78103, member85401)
3. **lambda_project_testdata_long.json** -- (all members)


*partial example of lambda_project_testdata_short.json:*
```
{
    "contents":[
       {
          "member_id":"member78103",
          "member_age":8,
          "member_sex":"F",
          "claim":[
             {
                "claim_id":"claim_22250000",
                "claim_type":"P",
                "type_of_bill":null,
                "admission_date":"2020-04-01",
                "discharge_date":"2020-04-01",
                "taxonomy_code":"2088P0231X",
                "place_of_service":2,
                "principle_diagnosis":"Q6231",
                "diagnosis_codes":[
                   "Q6231",
                   null,
                   null,
                   null,
                   null,
                   null,
                   null,
                   null,
                   null,
                   null
                ],
                "drg":null,
                "drg_severity":null,
                "drg_type":null,
                "claim_line":[
                   {
                      "line_number":1,
                      "from_date":"2020-04-01",
                      "thru_date":"2020-04-01",
                      "revenue_code":null,
                      "procedure_code":"99212",
                      "ndc_code":null,
                      "quantity":1,
                      "allowed_amount":79.2000000000000028
                   }
                ]
             },
             {
                "claim_id":"claim_39033600",
                "claim_type":"P",
                "type_of_bill":null,
                "admission_date":"2020-01-03",
                "discharge_date":"2020-01-03",
                "taxonomy_code":"363L00000X",
                "place_of_service":20,
                "principle_diagnosis":"J209",
                "diagnosis_codes":[
                   "J209",
                   "J029",
                   null,
                   null,
                   null,
                   null,
                   null,
                   null,
                   null,
                   null
                ],
                "drg":null,
                "drg_severity":null,
                "drg_type":null,
                "claim_line":[
                   {
                      "line_number":1,
                      "from_date":"2020-01-03",
                      "thru_date":"2020-01-03",
                      "revenue_code":null,
                      "procedure_code":"99214",
                      "ndc_code":null,
                      "quantity":1,
                      "allowed_amount":158.560000000000002
                   },
                   {
                      "line_number":2,
                      "from_date":"2020-01-03",
                      "thru_date":"2020-01-03",
                      "revenue_code":null,
                      "procedure_code":"87880",
                      "ndc_code":null,
                      "quantity":1,
                      "allowed_amount":6.61000000000000032
                   }
                ]
             },
...
```



<br/><br/>



<!-- Build Schema and Initialize --> 
<a name="code"></a>

## Code



<br/><br/>



<!-- CREDITS or ACKNOWLEDGEMENTS -->
<a name="credits"></a>

## Credits

| [![Community](https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRcnTCrjKmRCJDwebeZdr5iVQ_9QFHwtLEJsQ&usqp=CAU)](https://confluence.remedypartners.com/display/OA/Commercial+Analytics+Engineering)		|
|:---------------------------------------------------------------------------------------------------------:	|:------------------------------------------------------------------------------------------------------------------------------------------------:
|   **Opportunity Insights App Developemt Team!**                         			                          	    |
<br/><br/>

<!-- SUPPORT -->
<a name="support"></a>

## Support

Reach out at one of the following places:

- Slack Channel: 
- E-Mail: 
