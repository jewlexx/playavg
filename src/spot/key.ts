'use server';

import { cookies } from 'next/headers';

export interface SpotifyKey {
  key: string;
}

export async function setSpotifyKey(data: SpotifyKey) {
  cookies().set('spot-key', data.key);
}
