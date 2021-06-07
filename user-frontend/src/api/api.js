import { API_URL } from "./config";

export async function searchProvince() {
    try {
        let url = `${API_URL}/api/province/_search`
        const options = {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({})
        }
        let response = await fetch(url, options);
        let data = await response.json();
        return data
    } catch (err) {
        return null
    }
}