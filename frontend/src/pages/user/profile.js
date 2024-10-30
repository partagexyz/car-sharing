import React, { useContext } from 'react';
import UserProfile from '../../components/UserProfile';
import { NearContext } from '../../utils/near'

const UserProfilePage = () => {
    const { user } = useContext(NearContext);
    console.log("User data in UserProfilePage:", user);
    return <UserProfile user={user} />;
};

export default UserProfilePage;