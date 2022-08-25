import { gql } from '@apollo/client';

export const GET_USER = gql`
    query user($userId: Int!) {
        user(userId: $userId) {
            id
            name
        }
    }
`;


export const AUTHENTICATE_USER = gql`
    mutation authenticateUser($name: String!, $password: String!) {
        authenticateUser(input: {
            name: $name
            password: $password
        }) {
            id
            name
            password
        }
    }
`;