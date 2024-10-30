import React, { useContext, useEffect } from 'react';
import { NearContext } from '../utils/near';
import UserProfile from '@/components/UserProfile';

const UserProfilePage = () => {
    const { user } = useContext(NearContext);

    useEffect(() => {
        console.log("User updated:", user);
    }, [user]);

    return <UserProfile user={user} />;
};

export default UserProfilePage;