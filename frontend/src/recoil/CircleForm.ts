import { atom } from 'recoil';

export const circleFormState = atom({
    key: 'circleFormState',
    default: {
        organizationId: '',
        organizationName: '',
        familyName: '',
        givenName: '',
        studentId: '',
        email: '',
    }
});
