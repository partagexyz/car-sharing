import React from 'react';
import UserProfile from '../../components/UserProfile';

const UserProfilePage = () => {
    // fetch user data
    const user = { id: 'user_id', name: 'User Name', role: 'user' };
    return <UserProfile user={user} />;
};

export default UserProfilePage;