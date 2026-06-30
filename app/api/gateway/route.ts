import { NextResponse } from 'next/server'

export async function POST(request: Request) {
  try {
    const { authKey } = await request.json()
    const correctKey = process.env.GATEWAY_AUTH_KEY
    const protectedUrl = process.env.PROTECTED_DOC_URL

    if (authKey === correctKey) {
      return NextResponse.json({ success: true, url: protectedUrl })
    } else {
      return NextResponse.json({ success: false, message: 'Invalid authentication key. Access denied.' }, { status: 401 })
    }
  } catch (error) {
    return NextResponse.json({ success: false, message: 'Internal Server Error' }, { status: 500 })
  }
}
