import React from 'react';
import UserProfile from '../../components/UserProfile';
import { useContext } from 'react';
import { NearContext } from '../../utils/near'

const UserProfilePage = () => {
    const { user } = useContext(NearContext);
    return <UserProfile user={user} />;
};

export default UserProfilePage;