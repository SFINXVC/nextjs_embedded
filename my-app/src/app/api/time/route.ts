import { NextResponse } from 'next/server';

export const dynamic = 'force-dynamic';

export async function GET() {
  return NextResponse.json({
    serverTime: new Date().toISOString(),
    message: 'This is generated on the server'
  });
}
