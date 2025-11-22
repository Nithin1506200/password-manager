/**
 * TypeScript interface for the users table
 * Automatically generated from SQL schema
 */
export interface User {
  id: number;
  name: string;
  created_at: string; // ISO date string format
  pass_hash: string;
}

/**
 * Interface for creating a new user
 */
export interface NewUser {
  name: string;
  pass_hash: string;
}
