import axios from 'axios';

import { APIURL } from '../lib/url';

export const fetchData = async (
  url: string,
  method = 'GET',
  pyld?: Record<string, unknown>,
  formDataObj?: FormData,
) => {
  const fullUrl = url.startsWith('/') ? `${APIURL}${url}` : url;
  try {
    if (method === 'POST') {
      let dataToSend: FormData | Record<string, unknown>;
      const headers: Record<string, string> = {};

      if (formDataObj) {
        dataToSend = formDataObj;
        headers['Content-Type'] = 'multipart/form-data';
      } else if (pyld) {
        dataToSend = pyld;
        headers['Content-Type'] = 'application/json';
      } else {
        dataToSend = {};
      }

      const response = await axios.post(fullUrl, dataToSend, { headers });

      return {
        data: response.data,
        status: response.status,
      };
    }

    const response = await fetch(fullUrl, { method, next: { revalidate: 10 } });

    const data = await response.json(); // Assuming JSON response for GET
    return {
      data,
      status: response.status,
    };
  } catch (error: unknown) {
    throw new Error(String(error));
  }
};
