import React, { ComponentType } from 'react'
import Header from './components/Header.tsx'
import Footer from './components/Footer.tsx'
import './style/app.css'

import { useRouter } from 'aleph/react'

interface Metadata {
    title: string
    authors: string[]
    keywords?: string[]
    editable?: boolean
}

export default function App({ Page, pageProps }: { Page: ComponentType<any> & { meta: Metadata }, pageProps: Record<string, unknown> }) {
    const { routePath } = useRouter()
    const title = Page?.meta.title

    return (
        <div style={{height: "100%"}}>
            <head>
                <title>{title}</title>
                <meta name="viewport" content="width=605, initial-scale=1.0"></meta>
            </head>
            <Header title={title} />
            <div style={{display:"flex", width:"100%", alignItems:"center", padding: "10px"}}>
                <div style={{flexGrow: 1}} />
                <div style={{minWidth: "300px", maxWidth:"600px", flexGrow: 3}}>
                    <h1>{title}</h1>
                    <div style={{background:"white", padding: "30px"}}>
                        <Page {...pageProps} />
                    </div>
                </div>
                <div style={{flexGrow: 1}} />
            </div>
            <Footer Page={Page} />
        </div>
    )
}