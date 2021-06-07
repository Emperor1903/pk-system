import {API_URL} from "./config";

export async function
authenticate(username, password) {
    var url = `${API_URL}/api/auth/_login`;
    const options = {
        method: "POST",
        headers: {
            "Content-Type": "application/x-www-form-urlencoded",
        },
        body: `username=${username}&password=${password}`
    };
    var response = await fetch(url, options);
    var data = await response.text();
    return data;
}

export async function
getIdentity() {
    var url = `${API_URL}/api/auth/_me`;
    const options = {
        method: "POST",
    }
    try {
        var response = await fetch(url, options);
        var data = await response.json();
        return data;        
    } catch (err) {
        return null;
    }    
}

export async function
logout() {
    var url = `${API_URL}/api/auth/_logout`;
    var response = await fetch(url, {method: "POST"});
    var data = response.text();
    return data;
}
