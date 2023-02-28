# JSON Querying

## Description

A collection of use cases for querying our hash table that might inform a more optimal approach to search.

## Assumptions

- Our hash table is natively sorted by member and claim
- There is a natural relationship between sets we should try to leverage:
  - member -> claim -> claim line -> value 
- We can ignore entries in our hash table that are unrelated to our search criteria (similar to how a relational database can isolate rows that match the search parameters)

## Use cases

### __1. Finding related data elements__

_"I found keys associated with procedure codes and need to find the corresponding service dates."_

In this scenario, I may have the following results:

```json
{'contents.0.claim.29.claim_line.0.procedure_code': '99214', 'contents.0.claim.32.claim_line.0.procedure_code': '99214', 'contents.0.claim.46.claim_line.0.procedure_code': '99214', 'contents.0.claim.88.claim_line.0.procedure_code': '99214', 'contents.1.claim.9.claim_line.0.procedure_code': '99214', 'contents.1.claim.16.claim_line.0.procedure_code': '99214', 'contents.1.claim.17.claim_line.0.procedure_code': '99214', 'contents.1.claim.20.claim_line.0.procedure_code': '99214', 'contents.1.claim.32.claim_line.0.procedure_code': '99214', 'contents.1.claim.36.claim_line.0.procedure_code': '99214', 'contents.1.claim.47.claim_line.0.procedure_code': '99214', 'contents.2.claim.4.claim_line.0.procedure_code': '99214', 'contents.4.claim.8.claim_line.0.procedure_code': '99214', 'contents.5.claim.9.claim_line.0.procedure_code': '99214', 'contents.5.claim.12.claim_line.0.procedure_code': '99214', 'contents.6.claim.4.claim_line.0.procedure_code': '99214', 'contents.8.claim.1.claim_line.0.procedure_code': '99214'}
```

Querying the JSON for service dates relies on finding keys that match the member, claim, and line portion of the keys in the above results.

Matching portion of key (one example):

```json
'contents.0.claim.29.claim_line.0.'
```

A simple search through dict.keys() would evaluate every key in the hash table for a match. This is inefficient.

How can we leverage the origninal JSON? It might look something like this:

```python
# members is the array of members and their claims
# results is a dictionary of the keys with our office visit codes

from operator import itemgetter

ov_service_dates = []
for k, _ in results.items():
    # split the key to leverage the index values
    ix_mbr, ix_clm, ix_line = [int(i) for i in itemgetter(*[1,3,5])(k.split('.'))]
    # look up the from_date
    ov_service_dates.append(members[ix_mbr]['claim'][ix_clm]['claim_line'][ix_line]['from_date'])
```

This method introduces many operations to get to the value:

```python
members[ix_mbr]['claim'][ix_clm]['claim_line'][ix_line]['from_date']
```

Operations:

- list indexing: members[ix_mbr]
- dict lookup: ['claim']
- list indexing: [ix_clm]
- dict lookup:  ['claim_line']
- list indexing:  [ix_line]
- dict lookup: ['from_date']

Simply forming the key you want and continuing to use the hash table may prove to be more efficient:

```python
key = 'contents.1.claim.32.claim_line.0'  + '.from_date'
hash_table[key]
```

Operations:

- dict lookup: members['contents.1.claim.32.claim_line.0.from_date']

### __Solution__

Form the key and look up the corresponding entries in the hash table.

### __2. Chaining queries__

There will be scenarios where multiple search criteria are required to arrive at the data elements used in a metric. Health care utilization metrics certainly can have complex logic.

The first thing that comes to mind is method chaining. Something like:

```python
  hash_table.find('P').find('99214').find_elemenet('from_date')
```

This example takes a first search pass using the value `P` and a second pass using the value `99214`. The assumption is that the second query of the CPT code is acting on the _result set_ of the first query. In this way, chaining a query would search smaller and smaller result sets until we decide we're ready to extract a specific data element (`from_date`).




