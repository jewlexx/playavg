import { cookies } from 'next/headers';
import { NextRequest, NextResponse } from 'next/server';

function getAuthUrl() {
  const callback = process.env.CALLBACK_URL + '/api/login';
  const clientId = process.env.CLIENT_ID;

  return `https://accounts.spotify.com/authorize?client_id=${clientId}&response_type=code&redirect_uri=${callback}&scope=user-read-currently-playing+user-top-read`;
}

export async function GET(request: NextRequest) {
  const code = request.nextUrl.searchParams.get('code');

  console.log(code);

  if (code === null) {
    return NextResponse.redirect(getAuthUrl());
  } else {
    // Now + 1 hour, when the Spotify oauth code expires
    const expiry = Date.now() + 1000 * 60 * 60;
    const cookieStore = cookies();
    cookieStore.set({ name: 'spot-code', value: code, expiry } as any);

    return NextResponse.redirect(new URL('/', request.url));
  }
}
