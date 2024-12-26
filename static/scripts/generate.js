let codes = [
  107, 112, 110, 80, 70, 84, 97, 108, 72, 74, 85, 118, 111, 76, 103, 77, 67,
  118, 79, 76, 102, 87, 102, 109, 98, 105, 121, 101, 120, 82, 65, 89, 81, 119,
  83, 73, 75, 106, 100, 69, 115, 77, 86, 102, 98, 65, 81, 69, 67, 117, 114, 106,
  104, 106, 112, 107, 120, 98, 87, 88, 105, 81, 70, 73,
];

function createRandomString() {
  let str = "";
  str = codes
    .map((code) => {
      if (String.fromCharCode(code) === `\\`) {
        console.log(code);
      }

      return String.fromCharCode(code);
    })
    .join("");
  return str;
}

console.log(createRandomString());
