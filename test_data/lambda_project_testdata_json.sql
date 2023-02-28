-- **********************************************************************************
-- **********************************************************************************
-- 2022.05.02
-- gc
--
-- code for extract member, claim information from CDW to JSON
-- for lambda project
-- nothing elegant about this :/
--
--
-- **********************************************************************************
-- **********************************************************************************


-- in case we need this
create extension if not exists pgcrypto

-- nothing complicated; let's just picked some members randomly
-- we are not looking for a statistically sound sample across all categories


-- let's create a table in test so if we need to
-- modify the json, we have something a little more permanent to work with
-- we can delete this at the end of the project
drop table test.pl_sample_claims;
create table test.pl_sample_claims as
select
    member_id as member_id_org,
    'mbr_' || right(member_id, 9) as member_id,
    memberdob,
    date_part('year', admission_date) - date_part('year', memberdob) as member_age,
    membergender as member_sex,
    claim_id as claim_id_org,
    'clm_' || right(claim_id, 9) as claim_id,
    linenumber,
    claimtype,
    typeofbill,
    admission_date,
    discharge_date,
    from_date,
    thru_date,
    placeofservice,
    revenuecode,
    billingprovidertaxonomy,
    renderingprovidertaxonomy,
    principaldiagnosis,
    principalprocedure,
    array[diagnosis1, diagnosis2, diagnosis3, diagnosis4, diagnosis5, diagnosis6, diagnosis7, diagnosis8, diagnosis9, diagnosis10] as diagnosis_codes,
    procedurecode,
    procedurecodemodifier1,
    drg,
    drgseverity,
    drgtype,
    ndc,
    quantity,
    allowed_amount
from reporting.v2_cambia_normalized_claims
where member_id in (
                    select member_id
                    from reporting.v2_cambia_normalized_claims
                    where date_part('year', admission_date) = 2019
                    and claimtype in ('I', 'P')
                    order by random() limit 1000
                   );



create index test_pl_sample_claims_memberid on test.pl_sample_claims (member_id);
create index test_pl_sample_claims_claimid on test.pl_sample_claims (claim_id, linenumber);

-- 1000 members
-- 137709 claims
-- 314652 rows (claimlines)
/*
+----+---------+-------+--------+
|yr  |claimtype|clm_cnt|line_cnt|
+----+---------+-------+--------+
|2011|I        |15     |202     |
|2016|I        |261    |2378    |
|2016|P        |1159   |2400    |
|2016|R        |178    |178     |
|2017|I        |3015   |19644   |
|2017|P        |17537  |33450   |
|2017|R        |9050   |9050    |
|2018|I        |3887   |29927   |
|2018|P        |23393  |44334   |
|2018|R        |10462  |10466   |
|2019|I        |6476   |51822   |
|2019|P        |34738  |67132   |
|2019|R        |13220  |13224   |
|2020|I        |1481   |10191   |
|2020|P        |8398   |15441   |
|2020|R        |4812   |4813    |
+----+---------+-------+--------+
*/

select
    date_part('year', admission_date) as yr,
    claimtype,
    count(distinct claim_id) as clm_cnt,
    count(*) as clmline_cnt
from test.pl_sample_claims
group by date_part('year', admission_date) , claimtype
order by 1, 2;

select
    date_part('year', admission_date) as yr,
    claimtype,
    member_id,
    count(distinct claim_id) as clm_cnt,
    count(*) as clmline_cnt
from test.pl_sample_claims
group by member_id, date_part('year', admission_date) , claimtype
order by 1, 2, 3;


select date_part('year', admission_date) as yr, claimtype, count(distinct claim_id)
from reporting.v2_cambia_normalized_claims
group by date_part('year', admission_date), claimtype;



/*
============================================
let's create the JSON
based on:
 member
    claim
        claim_lines
============================================
*/

SELECT
	json_build_object('batch_id', 'long_allyears', 'sequence', 1, 'contents',
	json_agg(
		json_build_object(
                'member_id', m.member_id,
                'member_age', m.member_age,
                'member_sex', m.member_sex,
			    'claim', c.clm)
		)
	) member_json
FROM
    (
        select
          member_id,
          member_age,
          member_sex
        from test.pl_sample_claims
        group by member_id, member_age, member_sex
    ) m
JOIN (
        SELECT
        c.member_id,
        json_agg(
            json_build_object(
 --             'member_id', member_id,
                'claim_id', c.claim_id,
                'claim_type', c.claimtype,
                'type_of_bill', c.typeofbill,
                'admission_date', c.admission_date,
                'discharge_date', c.discharge_date,
                'taxonomy_code', c.billingprovidertaxonomy,
                'place_of_service', c.placeofservice,
                'principle_diagnosis', c.principaldiagnosis,
                'diagnosis_codes', c.diagnosis_codes,
                'drg', c.drg,
                'drg_severity', c.drgseverity,
                'drg_type', c.drgtype,
                'claim_line', cl.clmline
            )
	    ORDER BY member_id
	    ) clm
        FROM
             (select
                member_id,
                claim_id,
                claimtype,
                typeofbill,
                admission_date,
                discharge_date,
                billingprovidertaxonomy,
                placeofservice,
                principaldiagnosis,
                diagnosis_codes,
                drg,
                drgseverity,
                drgtype
              from test.pl_sample_claims
              group by
                    member_id,
                    claim_id,
                    claimtype,
                    typeofbill,
                    admission_date,
                    discharge_date,
                    billingprovidertaxonomy,
                    placeofservice,
                    principaldiagnosis,
                    diagnosis_codes,
                    drg,
                    drgseverity,
                    drgtype) c
        JOIN (
            SELECT
            claim_id,
            json_agg(
                json_build_object(
     --                'claim_id', cl.claim_id,
                     'line_number', cl.linenumber,
                     'from_date', cl.from_date,
                     'thru_date', cl.thru_date,
                     'revenue_code', cl.revenuecode,
                     'procedure_code', cl.procedurecode,
                     'ndc_code', cl.ndc,
                     'quantity', cl.quantity,
                     'allowed_amount', cl.allowed_amount
                )
            ORDER BY claim_id
            ) as clmline
            FROM
                 (select
                    claim_id,
                    linenumber,
                    from_date,
                    thru_date,
                    revenuecode,
                    procedurecode,
                    ndc,
                    quantity,
                    allowed_amount
                  from test.pl_sample_claims
                  order by claim_id, linenumber) cl
            GROUP BY claim_id) cl
    ON c.claim_id = cl.claim_id
    group by c.member_id) c
ON m.member_id = c.member_id
-- use the where to reduce the json set
-- short 2019
--   yr = 2019
--   claimtype I, P
--   clm_cnt < 4
--   limit 2
-- medium 2019
--   yr = 2019
--   claimtype I, P, R
--   clm_cnt < 4
--   limit 4
-- medium 2019
--   yr = 2019
--   claimtype I, P, R
--   clm_cnt < 4
--   limit 4
-- long 2019
--   yr = 2019
--   claimtype I, P, R
--   clm_cnt < 21
--   limit 10
-- long allyears
--   claimtype I, P, R
--   clm_cnt < 100
--   limit 25
where m.member_id in (
    select member_id
    from
        (select member_id, array_agg(distinct c.yr::int) as yr, array_agg(distinct claimtype) as claimtype, sum(clm_cnt)
    as clm_cnt
         from
            (select
                date_part('year', admission_date) as yr,
                claimtype,
                member_id,
                count(distinct claim_id) as clm_cnt
            from test.pl_sample_claims sc
            group by date_part('year', admission_date) , claimtype, member_id) c
        group by member_id) c
    where
        -- c.yr = array[2019] and
        c.claimtype && '{"I", "P"}'
        and clm_cnt < 100
        order by random() limit 25
);

select member_id, yr, claimtype, clm_cnt
from
    (select member_id, array_agg(distinct c.yr::int) as yr, array_agg(distinct claimtype) as claimtype, sum(clm_cnt)
as clm_cnt
     from
        (select
            date_part('year', admission_date) as yr,
            claimtype,
            member_id,
            count(distinct claim_id) as clm_cnt
        from test.pl_sample_claims sc
        group by date_part('year', admission_date) , claimtype, member_id) c
    group by member_id) c
where
    -- c.yr = array[2019] and
    c.claimtype && '{"I", "P"}'
    and clm_cnt < 100
    order by random() limit 25

