import { atom } from 'recoil';

export const circleFormState = atom({
    key: 'circleUpdateState',
    default: {
        organizationId: '',
        organizationName: '',
        familyName: '',
        givenName: '',
        studentId: '',
        email: '',
    }
});
