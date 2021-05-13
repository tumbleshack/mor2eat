var axios = require('axios');
const fs = require('fs')

var config = {
  method: 'get',
  url: 'https://locator.chick-fil-a.com.yext-cdn.com/search?q=31701&per=5',
  headers: { 
    'accept': 'application/json'
  }
};

const readZips = (fileName, zipCodeSet) => {
    const fileData = fs.readFileSync(fileName, "utf8");
    const zipArray = fileData.split(",\n");
    zipArray.forEach(element => {
        zipCodeSet.add(element);
    });
}

const downloadCfaMetadata = () => {
    var zipcodeSet = getZipCodeSet();
    console.log(zipcodeSet.size);
}

const getZipCodeSet = () => {
    var zipCodeSet = new Set();
    const states = [
      "ga",
      "nc",
      "sc",
      "al"
    ];

    states.forEach(state => {
      readZips("zipcodes/".concat(state, ".csv"), zipCodeSet);
    });

    return zipCodeSet;
}

// axios(config)
// .then(function (response) {
//     response.data
// })
// .catch(function (error) {
//     console.log(error);
// });

module.exports = { downloadCfaMetadata }