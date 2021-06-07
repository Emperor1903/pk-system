// import axios from "axios";
import { API_URL, TABLE_LIMIT } from "./config";
import {removeVietnameseTones} from "./utils";

async function
newDocument(name, body) {
    try {
        var url = `${API_URL}/admin/${name}/_new`;
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

export async function
createShifts() {
    try {
        var url = `${API_URL}/admin/shift/_new_week`;
        const options = {
            method: "POST",
        }
        var response = await fetch(url, options);
        var data = response.text();
        return data;
    } catch(err) {
        console.log(err);
        return null;
    }
}

async function
searchDocument(name, keyword, fields, ids, start, limit) {
    try {
        var url = new URL(`${API_URL}/admin/${name}/_search`);
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
                limit: limit
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
        var url = `${API_URL}/admin/${name}/_update`;
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
        var url = `${API_URL}/admin/${name}/_get`;
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
deleteDocument(name, id) {
    try {
        var url = `${API_URL}/admin/${name}/_delete`;
        const options = {
            method: "DELETE",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(id),
        };
        var response = await fetch(url, options);
        var data = await response.json();
        return data;
    } catch(err) {
        return null;
    }
}

export async function
newSpecializtion(name, desc) {
    var data = await newDocument("specialization", {
        name: name,
        desc: desc,
        search_keyword: removeVietnameseTones(name),
    });
}

export async function
newHospital(name, desc, province, address, phoneNum, images=[]) {
    var data = await newDocument("hospital", {
        name: name,
        desc: desc,
        province: province,
        address: address,
        phone_num: phoneNum,
        images: images,
        search_keyword: removeVietnameseTones(name + " " + address),
    });
    return data;
}

export async function
newClinic(
    name,
    desc,
    address,
    timeDesc,
    phoneNum,
    hospital,
    images=[],
) {
    var data = await newDocument("clinic", {
        name: name,
        desc: desc,
        address: address,
        time_desc: timeDesc,
        phone_num: phoneNum,
        hospital: hospital,
        images: images,
        search_keyword: removeVietnameseTones(name + " " + address),        
    });
    return data;
}

export async function
newProvince(name) {
    var data = await newDocument("province", {
        name: name,
        search_keyword: removeVietnameseTones(name),
    })
    return data;
}

export async function
newDoctor(
    name,
    shortIntro,
    intro,
    clinic,
    specialization,
    position,
    email,
    phoneNum,
    images,
) {
    var data = await newDocument("doctor", {
        name: name,
        short_intro: shortIntro,
        intro: intro,
        clinic: clinic,
        specialization: specialization,
        position: position,
        email: email,
        phone_num: phoneNum,
        images: images,
        keyword:  removeVietnameseTones(position + " " + name),
    });
    return data;
}

export async function
searchHospital(
    keyword,
    start,
    limit = TABLE_LIMIT
) {
    var data = await searchDocument("hospital", keyword, null, null, start, limit);
    return data;
}

export async function
searchClinic(
    keyword,
    hospital,
    start,
    limit = TABLE_LIMIT
) {
    var fields = ['hospital'];
    var ids = []
    if (hospital) {
        ids = [{"$oid": hospital}];
    }
    var data = await searchDocument("clinic",
                                    keyword,
                                    fields,
                                    ids,
                                    start,
                                    limit);
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
    var data = await searchDocument("doctor",
                                    keyword,
                                    fields,
                                    ids,
                                    start,
                                    limit);
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
    var data = await searchDocument("schedule", null, fields, ids, start, limit);
    return data;
}

export async function
searchShift(
    doctor,
    start,
    limit=TABLE_LIMIT
) {
    var fields = ["doctor"];
    var ids = [];
    if (doctor) {
        ids = [{"$oid": doctor}];
    }
    var data = await searchDocument("shift", null, fields, ids, start, limit);
    return data;
}


export async function
searchSpecialization(
    keyword,
    start,
    limit = TABLE_LIMIT
){
    let data = await searchDocument("specialization", keyword, null, null, start, limit);
    return data;
}

export async function
searchProvince() {
    var data = await searchDocument("province", null, null, null, 0, 1000);
    return data;
}







export async function
updateHospital(id, name, desc, province, address, phoneNum, images=[]) {
    var data = await updateDocument("hospital", id, {
        name: name,
        desc: desc,
        province: province,
        address: address,
        phone_num: phoneNum,
        images: images,
        search_keyword: removeVietnameseTones(name + " " + address),        
    });
}

export async function
updateDoctor(
    id,
    name,
    shortIntro,
    intro,
    clinic,
    specialization,
    position,
    email,
    phoneNum,
    images,
) {
    var data = await updateDocument(
        "doctor",
        id, {
            name: name,
            short_intro: shortIntro,
            intro: intro,
            clinic: clinic,
            specialization: specialization,
            position: position,
            email: email,
            phone_num: phoneNum,
            images: images,
            keyword:  removeVietnameseTones(position + " " + name),
        });
    return data;
}

export async function
newSchedule(
    doctor,
    clinic,
    shift_num,
    shift_day
) {
    var data = await newDocument("schedule", {
        doctor: doctor,
        clinic: clinic,
        shift_num: shift_num,
        shift_day: shift_day
    });
    return data;
}

export async function
updateClinic(
    id,
    name,
    desc,
    address,
    timeDesc,
    phoneNum,
    hospital,
    images=[],) {
    var data = await updateDocument("clinic", id, {
        name: name,
        desc: desc,
        address: address,
        time_desc: timeDesc,
        phone_num: phoneNum,
        hospital: hospital,
        search_keyword: removeVietnameseTones(name + " " + address),
        images: images,
    });
    return data;
}

export async function
updateSpecialization(
    id,
    name,
    desc) {
    var data = await updateDocument("specialization", id, {
        name: name,
        desc: desc,
        search_keyword: removeVietnameseTones(name),
    });
    return data;
}

export async function
updateProvince(
    id,
    name) {
    var data = await updateDocument("province", id, {
        name: name,
        search_keyword: removeVietnameseTones(name),
    })
    return data;
}

const UFormData = require('form-data');

export async function
uploadImage(file) {
    var key = "c1d5aef7b70c61daa8468c63e319f8e7"
    var url = `https://api.imgbb.com/1/upload?key=${key}`
    var data = new UFormData();
    data.append("image", file);
    try {
        var response = await fetch(url, {
            method: "POST",
            body: data,
        });
        var rs = await response.json();
        return rs;
    } catch(err) {
        console.log(err)
        return null;
    }
    
}
