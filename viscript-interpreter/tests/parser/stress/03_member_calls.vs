// ------------------------------------------------------------
// 3. Member access, optional member access, and calls
// ------------------------------------------------------------

let userName = user.name;
let userAge = user.age;

let maybeName = user?.name;
let maybeNestedName = nested.user?.name;

let firstCall = getValue();
let chainedCall = user.getName();
let chainedAccess = user.profile.name;