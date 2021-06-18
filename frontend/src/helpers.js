import axios from 'axios'

let host;
export const env = process.env.VUE_APP_ENVIRONMENT
export const version = process.env.VUE_APP_GIT_COMMIT
console.log(`env is ${env}`)
console.log(`version is: ${version}`)

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
