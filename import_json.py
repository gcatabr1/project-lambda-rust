import time
import json
from collections.abc import MutableMapping
from collections import defaultdict


def json_rd(p_filename: str):
    with open(p_filename, encoding='utf-8-sig') as f_in:
        return json.load(f_in)


def json_hash(p_dictionary, p_parent_key=False, p_sparse=False):
    _items = []
    for _key, _value in p_dictionary.items():
        # print(f'outer:  key: {_key}  value: {_value}')
        _new_key = str(p_parent_key) + '.' + _key if p_parent_key else _key
        if isinstance(_value, MutableMapping):
            # it's a dictionary
            # print(f'dict:  newkey: {_new_key}')
            if not _value.items():
                # check for empty dictionary
                # print(f'dict: add key value:  newkey: {_new_key}  value: {None}')
                _items.append((_new_key, None))
            else:
                # not empty, recurse!
                _items.extend(json_hash(_value, _new_key, p_sparse).items())
        elif isinstance(_value, list):
            # it's a list, so check to see if it's [not] empty
            # print(f'list:  newkey: {_new_key}')
            if len(_value):
                for _k, _v in enumerate(_value):
                    _items.extend(json_hash({str(_k): _v}, _new_key, p_sparse).items())
            else:
                # empty list
                # print(f'else list: add key value:  newkey: {_new_key}  value: {None}')
                _items.append((_new_key, None))
        else:
            # not dict or list, so append key, value
            if p_sparse is True and _value is None:
                    continue
            # print(f'else: add key value:  newkey: {_new_key}  value: {_value}')            
            _items.append((_new_key, _value))
    return dict(_items)


# def getKeysByValue(dictOfElements, valueToFind):
#     listOfKeys = list()
#     listOfItems = dictOfElements.items()
#     for item  in listOfItems:
#         if item[1] == valueToFind:
#             listOfKeys.append(item[0])
#     return  listOfKeys


def getKeysByValues(p_dictOfElements, p_listOfValues):
    _listOfKeys = list()
    _listOfItems = p_dictOfElements.items()
    for _item in _listOfItems:
        if _item[1] in p_listOfValues:
            _listOfKeys.append(_item[0])
    return  _listOfKeys

# ToDo: do getKeysByValues using regex

def getValuesByKeys(p_dictOfElements, p_listOfKeys, p_listOfSubKeys):
    _dictOfValues = dict()
    _listOfItems = p_dictOfElements.items()
    for _item in _listOfItems:
        # first check the claim header 
        # print(item[0].rsplit(".", 1)[0], item[0].rsplit(".", 1)[0].rsplit(".", 2)[0]) 
        if (_item[0].rsplit(".", 1)[0] in p_listOfKeys) or (_item[0].rsplit(".", 1)[0].rsplit(".", 2)[0] in p_listOfKeys): 
            # print(item[0].rsplit(".", 1)[0], item[0].rsplit(".", 1)[1])   
            if _item[0].rsplit(".", 1)[1] in p_listOfSubKeys:
                print(_item[0].rsplit(".", 1)[0], _item[0].rsplit(".", 1)[1])
                _dictOfValues[_item[0]] = _item[1]               
    return _dictOfValues


if __name__ == "__main__":
    _test_json = json_rd('test_data/lambda_project_testdata_short_with_headerblock.json')

    # _test_json = json_rd('/Users/gcattabriga/Downloads/escalatejson/experiment_bom_property_workflow_step_object_parameter.json')
    # print(test_json)
    # print(test_json['contents'][1]['member_id'])

    # for i in test_json['contents']:
    #     print(i['member_id'])
    _claimtype_list = ['I']
    _billtype_list = ['0110', '0111', '0120', '0121']
    _revcode_list = ['0100','0101','0102','0103','0104','0105','0106','0107','0108','0109','0110','0111','0112','0113','0114','0115','0116','0117','0118','0119','0120','0121','0122','0123','0124','0125','0126','0127','0128','0129','0130','0131','0132','0133','0134','0135','0136','0137','0138','0139','0140','0141','0142','0143','0144','0145','0218','0146','0147','0148','0149','0150','0151','0152','0153','0154','0155','0156','0157','0158','0159','0160','0161','0162','0163','0164','0165','0166','0167','0168','0169','0170','0171','0172','0173','0174','0175','0176','0177','0178','0179','0180','0181','0182','0183','0184','0185','0186','0187','0188','0189','0190','0191','0192','0193','0194','0195','0196','0197','0198','0199','0200','0201','0202','0203','0204','0205','0206','0207','0208','0209','0210','0211','0212','0213','0214','0215','0216','0217','0219']
    _subKeys = ['admission_date', 'discharge_date', 'allowed_amount']

    #start timer
    _start = time.time()

    _res = json_hash(_test_json, p_sparse=True)
    _end = time.time()
    # print(f'hashed json: {_res}')
    print(f'duration: {_end - _start}')

    with open('json_hash_output_short_python.json', 'w') as f:
        json.dump(_res, f, indent=2)

    # [s.rsplit(".", 1)[0] for s in keyclaimtype]
    # _keyclaimtype = [s.rsplit(".", 1)[0] for s in getKeysByValues(_res, _claimtype_list)]
    # _keybilltype = [s.rsplit(".", 1)[0] for s in getKeysByValues(_res, _billtype_list)]
    # _keyrevcode = [s.rsplit(".", 1)[0] for s in getKeysByValues(_res, _revcode_list)]  

    # print(f'claimtype_keys: {_keyclaimtype}')
    # print(f'billtype_keys: {_keybilltype}') 
    # print(f'revcode_keys: {_keyrevcode}')  

    # _key_intersec = set.intersection(set(_keyclaimtype), set(_keybilltype), set(_keyrevcode))
    # print(f'intersection: {_key_intersec}') 

    # print(getValuesByKeys(_res, ['contents.9.claim.1', 'contents.3.claim.34'], _subKeys))





