import { API_URL, TABLE_LIMIT } from "./config";
import {removeVietnameseTones} from "./utils";

function getRole() {
    return "guest";
}

async function
newDocument(name, body) {
    try {
        var role = getRole();
        var url = `${API_URL}/${role}/${name}/_new`;
        const options = {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(body),
        };
        var response = await fetch(url, options);
        var data = await response.json();
        return data;
    } catch (err) {
        return null;
    }
}

async function
searchDocument(
    name,
    keyword,
    fields,
    ids,
    start,
    limit,
    start_time,
    end_time) {
     try {
        keyword = removeVietnameseTones(keyword);
        var role = getRole();
        var url = `${API_URL}/${role}/${name}/_search`;        
        if (!keyword) keyword = null;
        if (!fields || !ids) {
            fields = null;
            ids = null;
        }
        if (!start) start = null;
        if (!limit) limit = null;
        const options = {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                keyword: keyword,
                ids: ids,
                fields: fields,
                start: start,
                limit: limit,
                start_time: start_time,
                end_time,
            }),
        };
        var response = await fetch(url, options);
        var data = await response.json();
        data = {
            total: data.total[0].count,
            data: data.data,
        };
        return data;
    } catch (err) {
        console.log(err);
        return null;
    }
}

async function
updateDocument(name, id, body) {
    try {
        var role = getRole();
        var url = `${API_URL}/${role}/${name}/_update`;
        const options = {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                id: id,
                data: body
            }),
        };
        var response = await fetch(url, options);
        var data = response.json();
        return data;
    } catch (err) {
        return null;
    }
}

export async function
getDocument(name, id) {
    try {
        var role = getRole();
        var url = `${API_URL}/${role}/${name}/_get`;        
        const options = {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(id),
        };
        var response = await fetch(url, options);
        var data = await response.json();
        return data;
    } catch(err) {
        console.log(err);
        return null;
    }
    
}

export async function
searchHospital(
    keyword,
    province,
    start,
    limit = TABLE_LIMIT
) {
    var fields = ['province'];
    var ids = [];
    if (province) {
        ids = [province];
    }
    var data = await searchDocument("hospital", keyword, fields, ids, start, limit, null, null);
    return data;
}

export async function
searchClinic(
    keyword,
    hospital,
    start,
    limit = TABLE_LIMIT
) {
    var fields = ["hospital"];
    var ids = [];
    if (hospital) {
        ids = [hospital];
    }
    var data = await searchDocument("clinic", keyword, fields, ids, start, limit, null, null);
    return data;
}

export async function
searchDoctor(
    keyword,
    clinic,
    start,
    limit = TABLE_LIMIT
) {
    var fields = ["clinic"];
    var ids = [];
    if (clinic) {
        ids = [{"$oid": clinic}];
    }
    var data = await searchDocument("doctor", keyword, fields, ids, start, limit, null, null);
    return data;
}


export async function
searchSchedule(
    doctor,
    start,
    limit=TABLE_LIMIT,
) {
    var fields = ["doctor"];
    var ids = [];
    if (doctor) {
        ids = [{"$oid": doctor}];
    }
    var data = await searchDocument("schedule", null, fields, ids, start, limit, null, null);
    return data;
}

export async function
searchShift(
    keyword,
    clinic,
    start,
    limit=TABLE_LIMIT,
    start_time=null,
) {
    var fields = ["clinic"];
    var ids = [];
    if (clinic) {
        ids = [{"$oid": clinic}];
    }
    var data = await searchDocument("shift", keyword, fields, ids, start, limit, start_time, null);
    return data;
}

export async function
searchSpecialization(
    keyword,
    start,
    limit = TABLE_LIMIT
){
    let data = await searchDocument("specialization", keyword, null, null, start, limit, null, null);
    return data;
}

export async function
searchProvince(keyword) {
    var data = await searchDocument("province", keyword, null, null, 0, 1000, null, null);
    return data;
}
