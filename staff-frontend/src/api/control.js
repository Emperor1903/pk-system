// import axios from "axios";
import {API_URL, TABLE_LIMIT} from "./config"

async function 
searchDocument(
    docName,
    keyword,
    start,
    limit
)
{
    var url = new URL(`${API_URL}/api/${docName}/_search`);
    const options = {
        method: "POST"
    }
    if(keyword) url.searchParams.append("keyword", keyword);
    if(start) url.searchParams.append("start", start);
    if(limit) url.searchParams.append("limit", limit);
    let response = await fetch(url, options);
    let data = await response.json();
    data = {
        "total": data.total[0].count,
        "data": data.data,
    }
    return data;
}



async function
newDocument(
    docName,
    docBody)
{
    var url = `${API_URL}/api/${docName}/_new`;
    const options = {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(docBody),
    };
    let response = await fetch(url, options);
    let data = await response.json();
    return data;
}

export async function
newHospital(
    name,
    desc,
    province,
    address,
    phoneNum
    ) 
{
    let data = await newDocument("hospital", {
        name: name,
        desc: desc,
        province: province,
        address: address,
        phone_num: phoneNum
    });
    return data;
}

export async function 
searchHospital(
    keyword,
    start,
    limit=TABLE_LIMIT
) 
{
    let data = searchDocument("hospital", keyword, start, limit);
    return data;
}