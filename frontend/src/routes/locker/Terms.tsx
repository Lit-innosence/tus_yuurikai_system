import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Checkbox, Button, Card, Layout } from 'antd';

const { Header, Content, Footer } = Layout;

const LockerTerms: React.FC = () => {
    const navigate  = useNavigate();
    // チェックボックスの状態を管理
    const [isChecked, setIsChecked] = useState(false); 

    const handleCheckboxChange = (e: any) => {
        setIsChecked(e.target.checked);  
    };

    const cardStyle: React.CSSProperties = {
        maxHeight: '100%',  
        overflowY: 'scroll', 
        padding: '16px',
    };

    const contentStyle: React.CSSProperties = {
        padding: '50px 50px',
        height: '70vh',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        flexDirection: 'column',
    };

    const termsContent = (
        <React.Fragment>
            <h3>利用規約</h3>
            <p>本利用規約（以下、「本規約」といいます）は、ユーザーが当社のサービスを利用する際に適用される条件を定めたものです。ユーザーは本規約に同意した上で、当社サービスを利用するものとします。</p>
            
            <h4>第1条（適用範囲）</h4>
            <p>
                本規約は、当社が提供する全てのサービスに適用されます。ユーザーが本規約に同意しない場合、当社のサービスを利用することはできません。
                本規約の変更がある場合、当社は合理的な方法で通知し、ユーザーは変更後の規約に従うものとします。
            </p>
            
            <h4>第2条（アカウントの管理）</h4>
            <p>
                ユーザーは、自己の責任においてアカウントを管理するものとします。アカウント情報の漏洩や不正使用があった場合、速やかに当社に通知してください。
                また、ユーザーは自己のアカウントを第三者に貸与・譲渡することはできません。
            </p>
            
            <h4>第3条（禁止事項）</h4>
            <p>
                ユーザーは、以下の行為を行ってはならないものとします：
                <ul>
                    <li>法令または公序良俗に反する行為</li>
                    <li>他のユーザーまたは第三者の権利を侵害する行為</li>
                    <li>虚偽の情報を提供する行為</li>
                    <li>当社のサービスの運営を妨害する行為</li>
                </ul>
                これらに違反した場合、当社はサービスの利用停止やアカウントの削除等の措置を取ることができます。
            </p>
    
            <h4>第4条（サービスの変更・停止）</h4>
            <p>
                当社は、ユーザーに事前通知することなく、サービスの内容を変更・中断・終了することができるものとします。これによりユーザーに生じた損害について、当社は一切の責任を負いません。
            </p>
    
            <h4>第5条（免責事項）</h4>
            <p>
                当社は、ユーザーが当社サービスを利用することによって発生した損害について、一切の責任を負わないものとします。
                また、当社サービスの提供に関連して発生したトラブルや紛争についても、当社は介入せず、ユーザー同士またはユーザーと第三者との間で解決するものとします。
            </p>
    
            <h4>第6条（著作権および知的財産権）</h4>
            <p>
                当社が提供するサービスに関する著作権および知的財産権は、全て当社または正当な権利者に帰属します。ユーザーは、これらの権利を侵害する行為を行ってはならないものとします。
            </p>
    
            <h4>第7条（プライバシーおよび個人情報の取り扱い）</h4>
            <p>
                当社は、ユーザーの個人情報を適切に取り扱うためのプライバシーポリシーを別途定めており、ユーザーはそれに同意した上でサービスを利用するものとします。
            </p>
    
            <h4>第8条（規約の変更）</h4>
            <p>
                当社は、必要に応じて本規約を変更することができるものとします。規約変更後、ユーザーが引き続きサービスを利用する場合、変更後の規約に同意したものとみなされます。
            </p>
    
            <h4>第9条（準拠法および管轄裁判所）</h4>
            <p>
                本規約は、日本法に準拠し解釈されます。また、本規約に関連する紛争が生じた場合、当社の所在地を管轄する裁判所を第一審の専属管轄裁判所とします。
            </p>
    
            <p>以上</p>
        </React.Fragment>
    );
    
    return (
        <Layout style={{ minHeight: '100vh' }}>
            <Header style={{ color: 'white', textAlign: 'center', backgroundColor: '#004ab3' }}>
                TUS YURIKAI SYSTEM
            </Header>
            <Content style={contentStyle}>
                <Card title="利用規約" bordered={true} style={cardStyle}>
                    {termsContent}
                </Card>
                <Checkbox onChange={handleCheckboxChange} style={{ marginTop: '16px' }}>
                    利用規約とプライバシーポリシーに同意します。
                </Checkbox>
                <Button 
                    type="primary" 
                    disabled={!isChecked}
                    style={{ marginTop: '16px' }}
                    onClick={() => { navigate('/locker-form') }}
                >
                    同意して次に進む
                </Button>
            </Content>
            <Footer style={{ textAlign: 'center', backgroundColor: 'white' }}>
                YURUKAI SYSTEM ©2024
            </Footer>
        </Layout>
    );
};

export default LockerTerms;
