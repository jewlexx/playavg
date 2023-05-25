import { NextResponse, type NextRequest } from 'next/server';

function getAuthUrl() {
  const callback = process.env.CALLBACK_URL + '/login';
  const clientId = process.env.CLIENT_ID;

  return `https://accounts.spotify.com/authorize?client_id=${clientId}&response_type=code&redirect_uri=${callback}&scope=user-read-currently-playing+user-top-read`;
}

export async function middleware(request: NextRequest) {
  const code = request.nextUrl.searchParams.get('code');
  if (code === null) {
    const url = getAuthUrl();
    return NextResponse.redirect(new URL(url, request.url));
  }

  return NextResponse.next();
}

export const config = {
  matcher: '/login',
};

export const runtime = 'nodejs';
