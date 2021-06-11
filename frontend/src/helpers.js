import axios from 'axios'

let host;
const env = process.env.APP_ENVIRONMENT

// set to prod in Dockerfile
if (env === 'prod') {
  host = "/backend"
} else {
  host = "http://127.0.0.1:5000"
}

export const fetchSecure = async function (path, options) {
  const {data} = await axios({
    method: 'get',
    url: `${host}/${path}`,
    // headers: {
    //   Authorization: `Bearer ${await token}`
    // },
    ...options,
  })
  return data
}

export const postSecure = async function (path, payload,options) {
  const {data} = await axios({
    method: 'post',
    url: `${host}/${path}`,
    data: payload,
    // headers: {
    //   Authorization: `Bearer ${await token}`
    // },
    ...options,
  })
  return data
}
