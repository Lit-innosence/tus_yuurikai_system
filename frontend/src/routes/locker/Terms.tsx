import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Checkbox, Button, Card, Layout } from 'antd';
import CustomHeader from '../../components/CustomHeader';
import CustomFooter from '../../components/CustomFooter';
import MovieButton from '../../components/MovieButton';

const { Content } = Layout;

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
            <h3>第1章 総則</h3>

            <h4>第1条（定義）</h4>
            <p>
                本規約におけるロッカーとは、東京理科大学葛飾友理会（以下、「友理会」といいます）が大学より管理を委託されている東京理科大学葛飾キャンパス講義棟内にあるロッカーのことを指します。
            </p>

            <h4>第2条（運営）</h4>
            <p>
                ロッカーの管理は友理会執行委員会がこれを行い、必要があれば大学当局の立会いのもと本規約に定めた様々な措置をとるものとします。
            </p>

            <h3>第2章 ロッカーの利用手続きについて</h3>

            <h4>第3条（利用対象者）</h4>
            <p>
                ロッカーの利用対象は、東京理科大学葛飾キャンパスに通う学部1年生から学部3年生までの学生とし、研究室に所属する4年生以上の学生には貸与を認めません。
            </p>

            <h4>第4条（申請方法）</h4>
            <p>
                ロッカーの利用手続きの方法については年度ごとに別途これを定めます。ただし、貸し出しは原則的に先着順とするため、希望する学生全員に貸し出せないことがあります。
            </p>

            <h4>第5条（申請条件）</h4>
            <p>
                ロッカーは1台につき学生2人で使用するものとし、原則的に2人1組で申請を行うものとします。
            </p>

            <h4>第6条（貸与期間）</h4>
            <p>
                ロッカーの貸与期間は、執行委員会が利用を許可した当該年度とし、来年度以降も継続して貸与を希望する場合は、その都度申請を行うものとします。
            </p>

            <h4>第7条（貸与期間後の撤去および点検）</h4>
            <p>
                利用期間終了後は、利用者は各自のロッカーの中身を撤去しなければなりません。
            </p>

            <h3>第3章 ロッカー利用について</h3>

            <h4>第8条（施錠義務）</h4>
            <p>
                利用者は、各自のロッカーに各自が用意した鍵で施錠しなければなりません。
            </p>

            <h4>第9条（点検）</h4>
            <p>
                友理会執行委員会は必要と認めた場合、ロッカー内の点検を行うことがあります。点検を行う際には事前にしかるべき方法で周知を行いますが、点検時に鍵がかかっていた場合、これを解錠することができるものとします。
            </p>

            <h4>第10条（盗難）</h4>
            <p>
                ロッカーにおいて盗難や破壊行為などが生じた場合、執行委員会は一切の責任を負いません。
            </p>

            <h4>第11条（撤去）</h4>
            <p>
                次の事項に該当する場合、友理会執行委員会が確認次第撤去を行い、適宜処分を行います。友理会執行委員会はこれによって生じた責任を一切負いません。
            </p>
            <ul>
                <li>ロッカーの上部および周辺に物が放置されていた場合</li>
                <li>ロッカーの利用の妨げになる場所に物が放置されていた場合</li>
                <li>ロッカーの不正利用が発覚した場合</li>
                <li>利用期間外を過ぎてもロッカーに物を収容していた場合</li>
                <li>ロッカーを故意に破損・汚損・改造する等著しい利用違反があった場合</li>
                <li>以下の保管禁止物を収容していた場合：
                    <ol>
                        <li>現金・貴重品</li>
                        <li>揮発性または爆発物等の危険品</li>
                        <li>生物</li>
                        <li>臭気を発するもの、腐敗変質しやすいもの、不潔なもの、ロッカーを汚損・き損するおそれのあるもの</li>
                        <li>法律で所持・携帯を禁じられているもの</li>
                        <li>その他保管に適さないと執行委員会が認めたもの</li>
                    </ol>
                </li>
            </ul>

            <h4>第12条（例外指示）</h4>
            <p>
                友理会執行委員会は、ロッカーの利用に際しロッカー利用者に指示を与えることができます。指示が与えられた場合、利用者はその指示に従わなければなりません。従わない場合、ロッカーの利用を禁止することがあります。
            </p>

            <h3>第4章 附則</h3>

            <h4>第13条（規約変更）</h4>
            <p>
                本規約は友理会執行委員会会議において審議の上、友理会執行委員の3分の2以上の賛成を得なければ変更することはできません。
            </p>

            <h4>第14条（施行日）</h4>
            <p>
                本規約は平成28年4月8日に公布され、同時に施行されます。
            </p>

        </React.Fragment>
    );
    
    return (
        <Layout style={{ minHeight: '100vh' }}>
            <CustomHeader />
            <MovieButton first={true}/>
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
                    onClick={() => { navigate('/locker/terms/important') }}
                >
                    同意して次に進む
                </Button>
            </Content>
            <CustomFooter />
        </Layout>
    );
};

export default LockerTerms;
