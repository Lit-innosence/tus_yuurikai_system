import { atom } from "recoil";

export const lockerFormState = atom({
    key: "lockerFormState",
    default: {
        studentId: "",
        lastName: "",
        firstName: "",
        coUserStudentId: "",
        coUserLastName: "",
        coUserFirstName: "",
    },
});
