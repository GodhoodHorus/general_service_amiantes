import { useMutation, useQuery } from '@apollo/client';
import { useState } from 'react';
import { AUTHENTICATE_USER, GET_USER } from '../gql/Query';

export default function User() {
    const { loading, error, data } = useQuery(GET_USER, {
        variables: {
            userId: 1
        }
    });

    // console.log(loading, error, data); // Stop at "true undefined undefined" 🥲

    if (loading) return null;
    if (error) return ( <div>Error ! {error.message}</div>);

    return (
        <div>
            {Object.values(data).map((user: any, index) => (
                <div key={index}>
                    <p>{ user.id }</p>
                    <p>{user.name}</p>
                </div>
            ))}
        </div>
    );
}


export function AuthenticateUser() {
    const [name, setName] = useState('');
    const [password, setPassword] = useState('');
    const [authenticateUser, { data, loading, error }] = useMutation(AUTHENTICATE_USER);

    if (loading) return null;
    if (error) return ( <div>Error ! {error.message}</div>);
    

    const handleAuthentication = (event: { preventDefault: () => void; }) => {
        event.preventDefault();

        let result = authenticateUser({
            variables: {
                name: name,
                password: password
            }
        });

        console.log(result);

        setName('');
        setPassword('');
    }

    return (
        <div className='flex place-content-center'>
            <form onSubmit={handleAuthentication}>
                <input
                    id="name"
                    name="name"
                    type="text"
                    onChange={event => setName(event.target.value)}
                    value={name}
                />
                <input
                    id="password"
                    name="password"
                    type="password"
                    value={password}
                    onChange={event => setPassword(event.target.value)}
                />
                <button type="submit">Authenticate</button>
            </form>
        </div>  
    );
}